use orst::*;
use rand::prelude::*;
use std::cell::Cell;
use std::cmp::Ordering;
use std::rc::Rc;

#[derive(Clone)]
struct SortEvaluator<T> {
    t: T,
    cmps: Rc<Cell<usize>>,
}

impl<T: PartialEq> PartialEq for SortEvaluator<T> {
    fn eq(&self, other: &Self) -> bool {
        self.cmps.set(self.cmps.get() + 1);
        self.t == other.t
    }
}

impl<T: PartialOrd> PartialOrd for SortEvaluator<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.cmps.set(self.cmps.get() + 1);
        self.t.partial_cmp(&other.t)
    }
}

impl<T: Ord> Ord for SortEvaluator<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cmps.set(self.cmps.get() + 1);
        self.t.cmp(&other.t)
    }
}

impl<T: Eq> Eq for SortEvaluator<T> {}

fn main() {
    let mut rand = rand::thread_rng();
    let counter = Rc::new(Cell::new(0));
    for &n in &[0, 1, 10, 100, 1_000, 10_000] {
        let mut values = Vec::with_capacity(n);
        for _ in 0..n {
            values.push(SortEvaluator {
                t: rand.gen::<usize>(),
                cmps: Rc::clone(&counter),
            })
        }

        for _ in 0..10 {
            values.shuffle(&mut rand);

            let (took, time) = bencher(BubbleSort, &values, &counter);
            println!("{} {} {} {}", "BUBBLE", n, took, time);
            let (took, time) = bencher(InsertionsSort { binary_search: true }, &values, &counter);
            println!("{} {} {} {}", "INSERTION-BINARY", n, took, time);
            let (took, time) = bencher(InsertionsSort { binary_search: false }, &values, &counter);
            println!("{} {} {} {}", "INSERTION-STANDARY", n, took, time);
            let (took, time) = bencher(SelectionSort, &values, &counter);
            println!("{} {} {} {}", "SELECTION", n, took, time);
            let (took, time) = bencher(QuickSort, &values, &counter);
            println!("{} {} {} {}", "QUICK", n, took, time);
        }
    }
}

fn bencher<T: Ord + Clone, S: Sorter>(sorter: S, values: &[SortEvaluator<T>], counter: &Cell<usize>) -> (usize, f64) {
    let mut values: Vec<SortEvaluator<T>> = values.to_vec();
    counter.set(0);
    let time = std::time::Instant::now();
    sorter.sort(&mut values);
    let took = time.elapsed();
    let count = counter.get();

    for i in 1..values.len() {
        assert!(values[i] >= values[i - 1]);
    }

    (
        count,
        took.as_secs_f64(),
    )
}