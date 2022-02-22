/*
    source: https://github.com/TheAlgorithms/Rust/blob/master/src/sorting/quick_sort.rs
    altered to implement Hoare partition scheme
    https://en.wikipedia.org/wiki/Quicksort#Hoare_partition_scheme
*/

use std::cmp::PartialOrd;

pub(crate) fn partition<T: PartialOrd + Clone>(arr: &mut [T], lo: usize, hi: usize) -> usize {
    let pivot = arr[((hi + lo) / 2)].clone();
    let mut left_b: usize = lo - 1;
    let mut right_b: usize = hi + 1;

    loop {
        //note: rust doesn't have do while loops, but
        //it does have unconditional loops
        loop {
            left_b += 1;
            if arr[left_b] >= pivot {
                break;
            }
        }

        loop {
            right_b -= 1;
            if arr[right_b] <= pivot {
                break;
            }
        }
        //no need to break out of the outer loop
        if left_b >= right_b {
            return right_b;
        } else {
            arr.swap(left_b, right_b);
        }
    }
}
fn _quick_sort<T: Ord + Clone>(arr: &mut [T], lo: usize, hi: usize) {
    if lo < hi {
        //lo >=0 && hi>=0 not necessary as the type is usigned
        let p = partition(arr, lo, hi);
        _quick_sort(arr, lo, p);
        _quick_sort(arr, p + 1, hi);
    }
}
pub fn quick_sort<T: Ord + Clone>(arr: &mut [T]) {
    let len = arr.len();
    _quick_sort(arr, 0, len - 1);
}
