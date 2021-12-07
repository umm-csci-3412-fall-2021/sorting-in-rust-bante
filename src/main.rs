use rand::{thread_rng, Rng};
use std::time::{Instant};

fn main() {
    // Feel free to raise size if you want to see the timing difference
    // between the different algorithms. Since insertion sort is O(N^2)
    // and the other two are O(N log N), you should definitely be able
    // to see a difference between it and the two faster algorithms.
    let size = 1000; // 100000;
    let v = generate_random_array(size, 0, size);

    let mut u = v.clone();
    let before_insertion = Instant::now();
    insertion_sort(&mut u);
    println!("Elapsed time for insertion sort was {:?}.", before_insertion.elapsed());

    let mut w = v.clone();
    // println!("{:?}", &w);
    let before_quicksort = Instant::now();
    quicksort(&mut w);
    println!("Elapsed time for quicksort was {:?}.", before_quicksort.elapsed());
    // println!("{:?}", &w);

    let before_merged = Instant::now();
    let merged_v = merge_sort(&v);
    println!("Elapsed time for merge sort was {:?}.", before_merged.elapsed());
    // println!("{:?}", v);
    // println!("{:?}", merged_v);
    println!("Is the original, random list in order?: {:?}", is_sorted(&v));
    println!("Was insertion sort in order?: {:?}", is_sorted(&u));
    println!("Was quicksort in order?: {:?}", is_sorted(&w));
    println!("Was merge sort in order?: {:?}", is_sorted(&merged_v));
}

// Insertion sort is "in place", so we modify the input array v
// directly and do _not_ return anything. The elements of the
// array need to traits `PartialOrd` (so they support < and ≤).
// Also requiring the trait `Debug` means you can print the array
// and slices of the array for debugging purposes with `{:?}`. I
// don't do that here, but you could add some print statements if,
// for example, you want to watch the bubbling happen.
//
// Note that the parameter v *has* to be mutable because we're 
// modifying it in place.
fn insertion_sort<T: PartialOrd + std::fmt::Debug>(v: &mut [T]) {
    // Goal: (All x, y | 0 ≤ x < y < length : v[x] ≤ v[y])
    for i in 0..v.len() {
        // Invariant: (All x, y | 0 ≤ x < y < i : v[x] ≤ v[y])
        // I.e., we assume everything < i is already sorted
        // by previous passes. Now we want to get everything
        // ≤ i to be sorted. This requires "bubbling" v[i]
        // to the left until it "finds its spot", i.e., until
        // swapping it one more time would make it _larger_
        // than the value to its right.
        //
        // j is where we are in the bubbling process, so we
        // start with j=i.
        let mut j = i;
        // If j > 0 we might still need to move left, so continue. 
        // But _only_ continue if v[j] _should_ move left, i.e.,
        // if it's less than the value to its left (so those two
        // are out of order.)
        while j > 0 && v[j-1] > v[j] {
            // Since j-1 and j are out of order swap them, and move
            // j one to the left to continue the bubbling if necessary.
            v.swap(j-1, j);
            j -= 1;
        }
    }
    // And we're done! The outer for loop is done O(N) times, and
    // the inner while loop is (on average) O(N), so insertion sort
    // is O(N^2).
}

// Quicksort sort is also "in place", so we modify the input array v
// directly and do _not_ return anything. The elements of the
// array need to traits `PartialOrd` (so they support < and ≤).
// Also requiring the trait `Debug` means you can print the array
// and slices of the array for debugging purposes with `{:?}`. I
// don't do that here, but you could add some print statements if,
// for example, you want to watch the sorting happen.
//
// Note that the parameter v *has* to be mutable because we're 
// modifying it in place.
fn quicksort<T: Ord>(arr: &mut [T]) {
    if arr.len() == 0 {
        return
    }
    perform_quick_sort(arr, 0, (arr.len() - 1) as isize);
}

fn perform_quick_sort<T: Ord>(arr: &mut [T], low: isize, high: isize) {
    if low < high {
        let p = partition(arr, low, high);
        perform_quick_sort(arr, low, p - 1);
        perform_quick_sort(arr, p + 1, high);
    }
}

fn partition<T: Ord>(arr: &mut [T], low: isize, high: isize) -> isize {
    let pivot = high as usize;
    let mut current = low - 1;
    let mut last = high;

    while true {
        current += 1;
        while arr[current as usize] < arr[pivot] {
            current += 1;
        }
        last -= 1;
        while last >= 0 && arr[last as usize] > arr[pivot] {
            last -= 1;
        }
        if current >= last {
            break;
        } else {
            arr.swap(current as usize, last as usize);
        }
    }
    arr.swap(current as usize, pivot as usize);
    current
}

// Merge sort can't be done "in place", so it needs to return a _new_
// Vec<T> of the sorted elements. The array elements need to have
// the traits `PartialOrd` and `Debug` like in the other sorting
// algorithms, but they also need to have the `Copy` trait so we
// can do things like `result.push(v[i])` to push element v[i] onto
// a vector result. This ends up copying v[i] (to prevent ownership
// issues on the array values), so we have to implement the `Copy`
// trait. Numbers all do this, so that should be fine.
// Note, however, that this has significant consequences – we can use `merge_sort`
// to sort things like numbers, but sorting "large" things (e.g., student records)
// would involve copying them, and that's likely to be expensive and perhaps undesirable.
//
// Note that here the parameter v does *not* have to be mutable because we're 
// creating and returning a new vector instead of modifying v in place.
// We're returning a vector instead of an array here because arrays have to
// know exactly how big they are. I suspect there's a way to make that work
// but I (Nic) couldn't figure out an easy way to sort out the types on the
// `merge()` function keeping everything as arrays. It was a lot easier to 
// just have the return type be Vec, so that's what I did. 
fn merge_sort<T: PartialOrd + std::marker::Copy + std::fmt::Debug>(v: &[T]) -> Vec<T> {
    // Merge sort is a recursive solution where we split the
    // array in half (slices make this easy), sort each half,
    // and then merge the results together. All the "interesting"
    // work is in the merge here, where in quicksort the "interesting"
    // work is in organizing around the pivot.

    let len = v.len();
    if len == 0 {
        return Vec::<T>::new();
    }
    if len == 1 {
        let mut result = Vec::<T>::new();
        result.push(v[0]);
        return result;
    }
    let middle = v.len() / 2; //rounds down by default
    let left = merge_sort(&v[0..middle]);
    let right = merge_sort(&v[middle .. len]);
    // Note that in Rust the last expression is what is
    // returned, and we don't need the explicit `return`
    // keyword. So this merges `left` and `right` and
    // returns the result as the result of this call to
    // `merge_sort()`.
    merge(left, right)
}

fn merge<T: PartialOrd + std::marker::Copy + std::fmt::Debug>(xs: Vec<T>, ys: Vec<T>) -> Vec<T> {
    let mut i = 0;
    let mut j = 0;
    let mut result = Vec::<T>::new();
    while i < xs.len() && j < ys.len(){
        if xs[i] < ys[j]{
            result.push(xs[i]);
            i += 1;
        }
        else {
            result.push(ys[j]);
            j += 1;
        }
    }
    while i < xs.len() {
        result.push(xs[i]);
        i = i +1;
    }
    while j < ys.len() {
        result.push(ys[j]);
        j = j +1;
    }

    result
}

fn is_sorted<T: PartialOrd>(slice: &[T]) -> bool {
    let len = slice.len();
    for i in 0..len-1{
        if slice[i] > slice[i+1]{
            return false;
        }
    }
    true
}

fn generate_random_array(len: i32, min: i32, max:i32) -> Vec<i32> {
    let mut rng = thread_rng();
    let mut v = Vec::new();
    for _i in 0..len{
        v.push(rng.gen_range(min, max));
    }
    // Rust returns the last expression in a function, so
    // this is equivalent to `return v`. 
    v
}

#[cfg(test)]
mod tests {
    use super::*;
    mod insertion_sort {
        use super::*;
        #[test]
        fn empty() {
            let mut input : [i32; 0] = [];
            insertion_sort(&mut input);
            let expected : [i32; 0] = [];

            assert_eq!(expected, input);
        }

        #[test]
        fn ten_items() {
            let mut input = [3, 2, 0, 5, 8, 9, 6, 3, 2, 0];
            insertion_sort(&mut input);
            let expected = [0, 0, 2, 2, 3, 3, 5, 6, 8, 9];

            assert_eq!(expected, input);
        }

        #[test]
        fn presorted() {
            let mut input = [0, 0, 2, 2, 3, 3, 5, 6, 8, 9];
            insertion_sort(&mut input);
            let expected = [0, 0, 2, 2, 3, 3, 5, 6, 8, 9];

            assert_eq!(expected, input);
        }
    }

    mod quicksort {
        use super::*;
        #[test]
        fn empty() {
            let mut input : [i32; 0] = [];
            quicksort(&mut input);
            let expected : [i32; 0] = [];

            assert_eq!(expected, input);
        }

        #[test]
        fn ten_items() {
            let mut input = [3, 2, 0, 5, 8, 9, 6, 3, 2, 0];
            quicksort(&mut input);
            let expected = [0, 0, 2, 2, 3, 3, 5, 6, 8, 9];

            assert_eq!(expected, input);
        }

        #[test]
        fn presorted() {
            let mut input = [0, 0, 2, 2, 3, 3, 5, 6, 8, 9];
            quicksort(&mut input);
            let expected = [0, 0, 2, 2, 3, 3, 5, 6, 8, 9];

            assert_eq!(expected, input);
        }
    }

    mod merge_sort {
        use super::*;
        #[test]
        fn empty() {
            let input : [i32; 0] = [];
            let result = merge_sort(&input);
            let expected : Vec<i32> = Vec::new();

            assert_eq!(expected, result);
        }

        #[test]
        fn ten_items() {
            let input = [3, 2, 0, 5, 8, 9, 6, 3, 2, 0];
            let result = merge_sort(&input);
            let expected = [0, 0, 2, 2, 3, 3, 5, 6, 8, 9].to_vec();

            assert_eq!(expected, result);
        }

        #[test]
        fn presorted() {
            let input = [0, 0, 2, 2, 3, 3, 5, 6, 8, 9];
            let result = merge_sort(&input);
            let expected = [0, 0, 2, 2, 3, 3, 5, 6, 8, 9].to_vec();

            assert_eq!(expected, result);
        }
    }
}