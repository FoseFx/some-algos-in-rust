#[allow(dead_code)]
mod sort;
mod problems;
fn main() {
    // println!("Quick Sort: {:?}", sort::quick::quick_sort(&mut vec![5, 1, 60, 30]));
    println!("Longest Consecutive Sequence: {:?}", problems::consecutive::longest_consecutive_sequence(&vec![1, 9, 3, 4, 10, 5, 2, 100]));
}
