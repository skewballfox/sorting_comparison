use crate::quick_sort::partition;
use crate::selection_sort::selection_sort;

pub fn hybrid_quick_sort<T>(arr: &mut [T])
where
    T: Ord + Clone,
{
    let len = arr.len();
    if len > 17 {
        _hybrid_quick(arr, 0, len - 1)
    } else {
        selection_sort(arr)
    }
}

fn _hybrid_quick<T>(arr: &mut [T], lo: usize, hi: usize)
where
    T: Ord + Clone,
{
    if lo < hi {
        if hi - lo > 17 {
            let p = partition(arr, lo, hi);
            _hybrid_quick(arr, lo, p);
            _hybrid_quick(arr, p + 1, hi);
        } else {
            selection_sort(&mut arr[lo..hi + 1]) //from first element to include to first element to exclude
        }
    }
}
