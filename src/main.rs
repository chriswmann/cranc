// Use the ProgresBar struct from lib.rs
use progrust_bar::bar::ProgressBarIteratorExt;


fn main() {
    println!("This is a test of the progress bar with a bound");
    let v = vec![1, 2, 3, 4, 5];
    for _ in v.iter().progress().with_bound() {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    std::thread::sleep(std::time::Duration::from_secs(1));

    println!("This is another test of the progress bar with a bound");
    for _ in (0..89).progress().with_bound() {
        std::thread::sleep(std::time::Duration::from_millis(20));
    }

    println!("This is a test of the progress bar without a bound");
    for i in (0..).progress() {
        std::thread::sleep(std::time::Duration::from_millis(20));
        if i == 79 {
            break;
        }
    }

    println!("This is a test of the progress bar without a bound that goes past 100");
    for i in (0..).progress() {
        std::thread::sleep(std::time::Duration::from_millis(20));
        if i == 179 {
            break;
        }
    }
}
