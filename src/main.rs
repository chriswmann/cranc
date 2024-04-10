use std::collections::HashSet;
use std::{thread::sleep, time::Duration};

const CLEAR: &str = "\x1B[2J\x1B[1;1H";

#[derive(Debug)]
struct Progress<Iter> {
    iter: Iter,
    i: usize,
    bound: Option<usize>,
}

impl<Iter> Progress<Iter> {
    pub fn new(iter: Iter) -> Self {
        Self {
            iter,
            i: 0,
            bound: None,
        }
    }
}

impl<Iter> Progress<Iter>
{
    fn format_bar(&self) -> String {
        match self.bound {
            Some(bound) => format!("[{}{}]", "*".repeat(self.i), " ".repeat(bound - self.i)),
            None => format!(" {}", "*".repeat(self.i)),
        }
    }
}

impl<Iter> Progress<Iter>
where
    Iter: ExactSizeIterator,
{
    pub fn with_bound(mut self) -> Self {
        self.bound = Some(self.iter.len());
        self
    }
}

impl<Iter> Iterator for Progress<Iter>
where
    Iter: Iterator,
{
    type Item = Iter::Item;

    fn next(&mut self) -> Option<Self::Item> {
        print!("{}", CLEAR);
        let bar = self.format_bar();
        println!("{}", bar);
        self.i += 1;
        self.iter.next()
    }
    
}

trait ProgressIteratorExt: Sized {
    fn progress(self) -> Progress<Self>;
}

impl<Iter> ProgressIteratorExt for Iter
where
    Iter: Iterator,
{
    fn progress(self) -> Progress<Self> {
        Progress::new(self)
    }
}

fn expensive_calculation(_n: &i32) {
    sleep(Duration::from_millis(400));
}

fn main() {
    let v = [1, 2, 3, 4, 5];
    for n in v.iter().progress().with_bound() {
        expensive_calculation(n);
    }
    expensive_calculation(&6);
    let mut h = HashSet::new();
    h.insert(0);
    h.insert(1);
    h.insert(2);
    h.insert(3);
    h.insert(4);
    for n in h.iter().progress().with_bound() {
        expensive_calculation(n);
    }
    for i in (0..).progress() {
        expensive_calculation(&i);
        if i == 6 {
            break;
        }
    }
}
