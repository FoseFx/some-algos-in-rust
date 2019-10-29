#[allow(dead_code)] mod sort;
#[allow(dead_code)] mod problems;
#[allow(dead_code)] mod exercises;

fn main() {
    // println!("Quick Sort: {:?}", sort::quick::quick_sort(&mut vec![5, 1, 60, 30]));
    // println!("Longest Consecutive Sequence: {:?}", problems::consecutive::longest_consecutive_sequence(&vec![1, 9, 3, 4, 10, 5, 2, 100]));
    // exercises::parallelism::spawn_threads(10);
    println!("{}", exercises::parallelism::do_calc(&vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30]));
}
