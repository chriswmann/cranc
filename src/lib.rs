#![deny(
    missing_docs,
    clippy::missing_docs_in_private_items,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc
)]
//! A simple progress indicator for iterators. Displays a spinner if the iterator is not sized.
//! For sized iterators, a progress bar is displayed by default.

/// The `bar` module contains the `ProgressBar` struct and its associated functions
pub mod bar {
    /// The escape sequence to clear the terminal and move the cursor to the top left
    const CLEAR: &str = "\x1B[2J\x1B[1;1H";

    /// A wrapper around an iterator that prints a progress bar
    #[derive(Debug)]
    pub struct ProgressIndicator<Iter> {
        /// The iterator to wrap.
        iter: Iter,
        /// A counter so we can track the progress of the iterator.
        i: usize,
        /// If the iterator is sized, bound stores the length of the iterator.
        bound: Option<usize>,
        /// The display width of the progress bar. TODO: Make this nicer, especially the
        /// relationship between this value and the actual size of the bar on screen.
        /// it.
        width: usize,
        /// The characters which are used to display the progress bar. These include partially
        /// filled blocks, e.g. `▍``and `▌`.
        bars: [char; 8],
        /// The number of characters in `bars`.
        nbars: usize,
        /// The characters used to display a spinner, which is used when the iterator isn't sized.
        /// The spinner rotates through the characters in the array.
        spinner: [char; 4],
        /// The number of characters in `spinner`.
        nspinners: usize,
    }

    /// Create a new `Progress` struct from an iterator
    impl<Iter> ProgressIndicator<Iter> {
        /// Create a new `Progress` struct from an iterator.
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

    /// Extension trait for iterators to enable the progress bar
    pub trait ProgressIndicatorIteratorExt: Sized {
        /// Add a progress indicator around an iterator
        fn progress(self) -> ProgressIndicator<Self>;
    }

    impl<Iter> ProgressIndicatorIteratorExt for Iter
    where
        Iter: Iterator,
    {
        fn progress(self) -> ProgressIndicator<Self> {
            ProgressIndicator::new(self)
        }
    }

    impl<Iter> Iterator for ProgressIndicator<Iter>
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

    impl<Iter> ProgressIndicator<Iter> {
        /// Format the progress indicator for the current iteration
        /// Use a bar if the the iterator's size has been used to define
        /// ProgressIndicator.bound, otherwise use a spinner.
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
                        format!(
                            " {:6.2}% [{}{}{}]",
                            percent,
                            "█".repeat(whole_bars),
                            part_bar,
                            " ".repeat(remaining)
                        )
                    }
                }
                None => format!("{:8}it {}", self.i, self.spinner[self.i % self.nspinners]),
            }
        }
    }

    impl<Iter> ProgressIndicator<Iter>
    where
        Iter: ExactSizeIterator,
    {
        /// Set the bound of the progress bar to the length of the iterator.
        ///
        /// # Example
        ///
        /// ```
        /// use cranc::ProgressIndicator;
        ///
        /// let v = &[1, 2, 3];
        /// let p = v.iter().progress().bar();
        /// for i in p {
        ///     println!("{}", i);
        /// }
        /// ```
        pub fn bar(mut self) -> Self {
            self.bound = Some(self.iter.len());
            self
        }

        /// Use a bounded progress bar with a specific display width
        ///
        /// # Example
        ///
        /// ```
        /// use cranc::ProgressIndicator;
        ///
        /// let v = &[1, 2, 3];
        /// let p = v.iter().progress().bar_custom_width(10);
        /// for i in p {
        ///     println!("{}", i);
        /// }
        /// ```
        pub fn bar_custom_width(mut self, width: usize) -> Self {
            self.bound = Some(self.iter.len());
            self.width = std::cmp::max(width, 1);
            self
        }

        /// Use a spinner with a sized iterator instead. This overrides
        /// the default behaviour of `progress()`, which is to use a bar for sized iterators.
        ///
        /// # Example
        ///
        /// ```
        /// use cranc::ProgressIndicator;
        ///
        /// let v = &[1, 2, 3];
        /// let p = v.iter().progress().spinner();
        /// for i in p {
        ///     println!("{}", i);
        /// }
        /// ```
        pub fn spinner(self) -> Self {
            self
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn iterator_still_iterates() {
            let v = &[1, 2, 3];
            let mut p = v.iter().progress().bar();
            assert_eq!(p.next(), Some(&1));
            assert_eq!(p.next(), Some(&2));
            assert_eq!(p.next(), Some(&3));
            assert_eq!(p.next(), None);
        }

        #[test]
        fn format_empty_sized_iter() {
            let v: Vec<u8> = Vec::new();
            let p = v.iter().progress().bar();
            assert_eq!(p.format_bar().len(), 19);
        }

        #[test]
        fn format_sized_iter() {
            let v = &[1];
            let p = v.iter().progress().bar();
            assert!(p.format_bar().starts_with(' '));
            assert!(p.format_bar().contains('%'));
        }

        #[test]
        fn format_with_width() {
            let v = &[1, 2, 3];
            let p = v.iter().progress().bar_custom_width(10);
            assert!(p.format_bar().contains('%'));
            assert_eq!(p.format_bar().len(), 91);
            assert_eq!(p.width, 10);
            assert_eq!(p.nbars, 8);
        }

        #[test]
        fn progress_bar_with_width() {
            let v = &[1, 2, 3];
            let p = v.iter().progress().spinner();
            assert!(p.format_bar().starts_with(' '));
            assert!(p.format_bar().contains("it"));
        }

        #[test]
        fn format_unsized_iter() {
            let v = [0..];
            let p = v.iter().progress();
            assert!(p.format_bar().starts_with(' '));
            assert!(p.format_bar().contains("it"));
        }
    }
}

pub use bar::ProgressIndicatorIteratorExt as ProgressIndicator;
