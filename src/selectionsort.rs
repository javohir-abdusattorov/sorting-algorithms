use super::Sorter;

pub struct SelectionSort;

impl Sorter for SelectionSort {
    fn sort<T>(&self, slice: &mut [T]) where T: Ord {
        for unsorted in 0..slice.len() {
            let smallest = slice[unsorted..]
                .iter()
                .enumerate()
                .min_by_key(|t| t.1)
                .map(|(i, _)| unsorted + i)
                .expect("empty slice, cannot be sorted");

            if unsorted != smallest {
                slice.swap(unsorted, smallest);
            }
        }
    }
}


#[test]
fn it_sorts() {
    let mut things = vec![4, 2, 5, 3, 1];
    SelectionSort.sort(&mut things);

    assert_eq!(
        things,
        &[1, 2, 3, 4, 5]
    );
}