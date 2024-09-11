use super::Sorter;

pub struct InsertionsSort {
    pub binary_search: bool,
}

impl Sorter for InsertionsSort {

    /// Insertion sort splits up slice into 2 lists: [ sorted | not sorted ]
    /// and starts from beginning to end adding from not sorted to sorted one by one
    fn sort<T>(&self, slice: &mut [T]) where T: Ord {
        if self.binary_search {
            for unsorted in 1..slice.len() {
                // slice[unsorted..] is not sorted yet
                // take slice[unsorted] and place in sorted location in slice[..=unsorted]
                let mut i = unsorted;
                while i > 0 && slice[i - 1] > slice[i] {
                    slice.swap(i - 1, i);
                    i -= 1;
                }
            }
        }
        else {
            for unsorted in 1..slice.len() {
                let index = match slice[..unsorted].binary_search(&slice[unsorted]) {
                    // [a, c, e].binary_search(c) => Ok(1)
                    Ok(i) => i,
                    // [a, c, e].binary_search(b) => Err(1)
                    Err(i) => i,
                };
    
                slice[index..=unsorted].rotate_right(1);
            }
        }
        
    }
}


#[test]
fn it_sorts_standart() {
    let mut things = vec![4, 2, 5, 3, 1];
    let sorter = InsertionsSort { binary_search: false };
    sorter.sort(&mut things);

    assert_eq!(
        things,
        &[1, 2, 3, 4, 5]
    );
}

#[test]
fn it_sorts_binary() {
    let mut things = vec![4, 2, 5, 3, 1];
    let sorter = InsertionsSort { binary_search: true };
    sorter.sort(&mut things);

    assert_eq!(
        things,
        &[1, 2, 3, 4, 5]
    );
}