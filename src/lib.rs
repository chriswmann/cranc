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
        bars: [char; 8],
        nbars: usize,
        spinner: [char; 4],
        nspinners: usize,
    }
    
    /// Create a new `Progress` struct from an iterator
    impl<Iter> ProgressBar<Iter> {
    
        /// Create a new 
        pub fn new(iter: Iter) -> Self {
            Self {
                iter,
                i: 0,
                bound: None,
                width: 1,
                bars: [' ', '▏', '▎', '▍', '▌', '▋', '▊', '▉'],
                nbars: 8,
                spinner: ['-', '\\', '|', '/'],
                nspinners: 4,
            }
        }
    }
    
    impl<Iter> ProgressBar<Iter>
    {
        fn format_bar(&self) -> String {
            match self.bound {
                Some(bound) => {
                    let width = self.width * self.nbars;
                    let percent = (self.i as f64 / bound as f64) * 100.0;
                    let stepsize = width as f64 / bound as f64;
                    let whole_bars = (self.i as f64 * stepsize).floor() as usize;
                    let part_bar = self.bars[self.i % stepsize.ceil() as usize];
                    let remaining = (width - whole_bars).saturating_sub(1);
                    if whole_bars == width {
                        format!(" {:6.2}% [{}]", percent, "█".repeat(whole_bars))
                    } else {
                        format!(" {:6.2}% [{}{}{}]", percent, "█".repeat(whole_bars), part_bar, " ".repeat(remaining))
                    }
                },
                None => format!("{:8}it {}", self.i, self.spinner[self.i % self.nspinners]),
            }
        }
    }
    
    impl<Iter> ProgressBar<Iter>
    where
        Iter: ExactSizeIterator,
    {
        /// Set the bound of the progress bar to the length of the iterator.
        /// This enables a more informative display of progress.
        pub fn sized(mut self) -> Self {
            self.bound = Some(self.iter.len());
            self
        }

        /// Use a bounded progress bar with a specific width
        pub fn sized_custom_width(mut self, width: usize) -> Self {
            self.width = std::cmp::max(width, 1);
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
        fn progress_adapted_iterator_still_iterates() {
            let v = vec![1, 2, 3];
            let mut p = v.iter().progress().sized();
            assert_eq!(p.next(), Some(&1));
            assert_eq!(p.next(), Some(&2));
            assert_eq!(p.next(), Some(&3));
            assert_eq!(p.next(), None);
        }

        #[test]
        fn progress_bar_format_empty_sized_iter() {
            let v: Vec<u8> = Vec::new();
            let p = v.iter().progress().sized();
            assert_eq!(p.format_bar().len(), 19);
        }

        #[test]
        fn progress_bar_format_sized_iter() {
            let v = vec![1];
            let p = v.iter().progress().sized();
            assert_eq!(p.format_bar().starts_with(" "), true);
            assert_eq!(p.format_bar().contains("%"), true);
            assert_ne!(p.format_bar().contains("it"), true);
        }
        
        #[test]
        fn progress_bar_with_width() {
            let v = vec![1, 2, 3];
            let p = v.iter().progress().sized_custom_width(10);
            assert_eq!(p.format_bar().len(), 12);
            assert_eq!(p.width, 10);
            assert_eq!(p.nbars, 8);
        }

        #[test]
        fn progress_bar_format_unsized_iter() {
            let v = [0..];
            let p = v.iter().progress();
            assert_eq!(p.format_bar().starts_with(" "), true);
            assert_eq!(p.format_bar().contains("it"), true);
            assert_ne!(p.format_bar().contains("%"), true);
        }
    }
}
