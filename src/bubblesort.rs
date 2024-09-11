use super::Sorter;

pub struct BubbleSort;

impl Sorter for BubbleSort {
    fn sort<T>(&self, slice: &mut [T]) where T: Ord {
        let mut swapped = true;
        while swapped {
            swapped = false;

            for i in 1..slice.len() {
                if slice[i - 1] > slice[i] {
                    slice.swap(i - 1, i);
                    swapped = true;
                }
            }
        }
    }
}


#[test]
fn it_sorts() {
    let mut things = vec![4, 2, 5, 3, 1];
    BubbleSort.sort(&mut things);

    assert_eq!(
        things,
        &[1, 2, 3, 4, 5]
    );
}