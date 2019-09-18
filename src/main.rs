mod sort;
fn main() {
    println!("Quick Sort: {:?}", sort::quick::quick_sort(&mut vec![5, 1, 60, 30]));
}
