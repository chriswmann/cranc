#![deny(missing_docs)]
#![deny(clippy::missing_docs_in_private_items)]
#![deny(rustdoc::broken_intra_doc_links)]
//! A simple progress bar for iterators

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

