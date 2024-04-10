// Use the ProgresBar struct from lib.rs
use progrust_bar::bar::ProgressBarIteratorExt;


fn main() {
    let v = vec![1, 2, 3, 4, 5];
    for _ in v.iter().progress().with_bound() {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    std::thread::sleep(std::time::Duration::from_secs(1));

    for _ in (0..100).progress().with_bound() {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
