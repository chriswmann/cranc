use std::{time::Duration, thread::sleep};
use std::collections::HashSet;

const CLEAR: &str = "\x1B[2J\x1B[1;1H";

struct Progress<Iter> {
    iter: Iter,
    i: usize,
    bound: Option<usize>,
}

impl<Iter> Progress<Iter> {
    pub fn new(iter: Iter) -> Self {
        Self { iter, i: 1, bound: None }
    }
}

impl<Iter> Progress<Iter> 
where Iter: ExactSizeIterator
{
    pub fn with_bound(mut self) -> Self {
        self.bound = Some(self.iter.len());
        self
    }
}

impl<Iter> Iterator for Progress<Iter>
where Iter: Iterator
{
    type Item = Iter::Item;

    fn next(&mut self) -> Option<Self::Item> {
        print!("{}", CLEAR);
        match self.bound {
            Some(bound) => {
                println!("[{}{}]", "*".repeat(self.i), " ".repeat(bound - self.i));
                self.i += 1;
            },
            None => {
                println!("{}", "*".repeat(self.i));
                self.i += 1;
            }
        }
        print!("{}", CLEAR);
        println!("[{}{}]", "*".repeat(self.i), " ".repeat(5 - self.i));
        self.i += 1;
        self.iter.next()
    }
}

trait ProgressIteratorExt: Sized
{
    fn progress(self) -> Progress<Self>;
}

impl<Iter> ProgressIteratorExt for Iter
where Iter: Iterator
{
    fn progress(self) -> Progress<Self> {
        Progress::new(self)
    }
}

fn expensive_calculation(_n: &i32) {
    sleep(Duration::from_millis(400));
}

fn main() {
    let v = vec![1, 2, 3, 4, 5];
    println!("Calculating first...");
    sleep(Duration::from_secs(1));
    for n in v.iter().progress().with_bound() {
        expensive_calculation(n);
    }
    println!("Finished calculating first!");
    sleep(Duration::from_secs(1));
    let mut h = HashSet::new();
    h.insert(0);
    h.insert(1);
    h.insert(2);
    h.insert(3);
    h.insert(4);
    println!("Calculating second...");
    sleep(Duration::from_secs(1));
    for n in h.iter().progress().with_bound() {
        expensive_calculation(n);
    }
    println!("Finished calculating second!");
    sleep(Duration::from_secs(1));
    println!("Calculating third...");
    sleep(Duration::from_secs(1));
    for i in (0..).progress().take(5) {
        expensive_calculation(&i);
    }
    println!("Finished calculating third!");
    sleep(Duration::from_secs(1));
}
