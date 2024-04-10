#![deny(
    missing_docs,
    clippy::missing_docs_in_private_items,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc
)]
//! A simple progress bar for iterators

const CLEAR: &str = "\x1B[2J\x1B[1;1H";

/// A wrapper around an iterator that prints a progress bar
#[derive(Debug)]
pub struct ProgressBar<Iter> {
    iter: Iter,
    i: usize,
    bound: Option<usize>,
}

/// Create a new `Progress` struct from an iterator
impl<Iter> ProgressBar<Iter> {

    /// Create a new 
    pub fn new(iter: Iter) -> Self {
        Self {
            iter,
            i: 0,
            bound: None,
        }
    }
}

impl<Iter> ProgressBar<Iter>
{
    fn format_bar(&self) -> String {
        match self.bound {
            Some(bound) => format!("[{}{}]", "*".repeat(self.i), " ".repeat(bound - self.i)),
            None => format!(" {}", "*".repeat(self.i)),
        }
    }
}

impl<Iter> ProgressBar<Iter>
where
    Iter: ExactSizeIterator,
{
    /// Set the bound of the progress bar to the length of the iterator.
    /// This enables a more informative display of progress.
    pub fn with_bound(mut self) -> Self {
        self.bound = Some(self.iter.len());
        self
    }
}

impl<Iter> Iterator for ProgressBar<Iter>
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

trait ProgressBarIteratorExt: Sized {
    fn progress(self) -> ProgressBar<Self>;
}

impl<Iter> ProgressBarIteratorExt for Iter
where
    Iter: Iterator,
{
    fn progress(self) -> ProgressBar<Self> {
        ProgressBar::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_still_iterates() {
        let v = vec![1, 2, 3, 4, 5];
        let mut p = v.iter().progress().with_bound();
        assert_eq!(p.next(), Some(&1));
        assert_eq!(p.next(), Some(&2));
        assert_eq!(p.next(), Some(&3));
        assert_eq!(p.next(), Some(&4));
        assert_eq!(p.next(), Some(&5));
        assert_eq!(p.next(), None);
    }
}
