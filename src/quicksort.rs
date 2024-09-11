use super::Sorter;

pub struct QuickSort;

impl Sorter for QuickSort {
    fn sort<T>(&self, slice: &mut [T]) where T: Ord {
        quicksort(slice)
    }
}

fn quicksort<T: Ord>(slice: &mut [T]) {
    match slice.len() {
        0 | 1 => return,
        2 => {
            if slice[0] > slice[1] {
                slice.swap(0, 1);
            }
            return
        },
        _ => {}
    }

    let (pivot, rest) = slice.split_first_mut().expect("slice is non-empty");
    let mut left = 0;
    let mut right = rest.len() - 1;

    while left <= right {
        if &rest[left] <= pivot {
            // left is already on the correct side
            left += 1;
        }
        else if &rest[right] > pivot {
            if right == 0 {
                break;
            }
            // right is already on the correct side
            right -= 1;
        }
        else {
            // swap left and right, and increment them both
            rest.swap(left, right);
            left += 1;
            if right == 0 {
                break;
            }
            right -= 1;
        }
    }

    // re-align left and right to account for pivot
    let left = left + 1;

    // place the pivot at its final location
    slice.swap(0, left - 1);

    // split_at_mut(mid: usize) -> (&mut [..mid], &mut [mid..])
    let (left, right) =  slice.split_at_mut(left - 1);
    assert!(left.last() <= right.first());

    quicksort(left);
    quicksort(&mut right[1..]);
}


#[test]
fn it_sorts() {
    let mut things = vec![4, 2, 5, 3, 1];
    QuickSort.sort(&mut things);

    assert_eq!(
        things,
        &[1, 2, 3, 4, 5]
    );
}