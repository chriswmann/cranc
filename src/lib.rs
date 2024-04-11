#![deny(
    missing_docs,
    clippy::missing_docs_in_private_items,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc
)]
//! A simple progress bar for iterators

/// The `bar` module contains the `ProgressBar` struct and its associated functions
pub mod bar {
    const CLEAR: &str = "\x1B[2J\x1B[1;1H";
    
    /// A wrapper around an iterator that prints a progress bar
    #[derive(Debug)]
    pub struct ProgressBar<Iter> {
        iter: Iter,
        i: usize,
        bound: Option<usize>,
        width: usize,
    }
    
    /// Create a new `Progress` struct from an iterator
    impl<Iter> ProgressBar<Iter> {
    
        /// Create a new 
        pub fn new(iter: Iter) -> Self {
            Self {
                iter,
                i: 0,
                bound: None,
                width: 100,
            }
        }
    }
    
    impl<Iter> ProgressBar<Iter>
    {
        fn format_bar(&self) -> String {
            match self.bound {
                Some(bound) => {
                    let percent = self.i as f64 / bound as f64 * self.width as f64;
                    let done = percent as usize;
                    let remaining = self.width - done;
                    format!(" {:6.2}/100% [{}{}]", percent, "*".repeat(done as usize), " ".repeat(remaining))
                },
                None => format!("{:12} {}", self.i, ["-", "\\", "|", "/"][self.i % 4]),
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

        /// Use a bounded progress bar with a specific width
        pub fn with_sized_bound(mut self, width: usize) -> Self {
            self.bound = Some(self.iter.len());
            self.width = width;
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
    
    /// Extension trait for iterators to enable the progress bar
    pub trait ProgressBarIteratorExt: Sized {
        /// Add a progress bar around an iterator
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
        fn test_progress_adapted_iterator_still_iterates() {
            let v = vec![1, 2, 3];
            let mut p = v.iter().progress().with_bound();
            assert_eq!(p.next(), Some(&1));
            assert_eq!(p.next(), Some(&2));
            assert_eq!(p.next(), Some(&3));
            assert_eq!(p.next(), None);
        }

        #[test]
        fn test_progress_bar_format_empty_bounded_iter() {
            let v: Vec<u8> = Vec::new();
            let p = v.iter().progress().with_bound();
            assert_eq!(p.format_bar().starts_with(" "), true);
            assert_eq!(p.format_bar().contains("100%"), true);
        }

        #[test]
        fn test_progress_bar_format_bounded_iter() {
            let v = vec![1];
            let p = v.iter().progress().with_bound();
            assert_eq!(p.format_bar().starts_with(" "), true);
            assert_eq!(p.format_bar().contains("100%"), true);
        }

        #[test]
        fn test_progress_bar_format_unbounded_iter() {
            let v = [0..];
            let p = v.iter().progress();
            assert_eq!(p.format_bar().starts_with(" "), true);
            assert_eq!(p.format_bar().contains(|x| x == '\\' || x == '|' || x == '-' || x == '/'), true);
        }
    }
}
