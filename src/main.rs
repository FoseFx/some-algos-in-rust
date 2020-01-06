
#[allow(dead_code)] mod sort;
#[allow(dead_code)] mod problems;
#[allow(dead_code)] mod exercises;
#[allow(dead_code)] mod graphs;

fn main() {
    // println!("Quick Sort: {:?}", sort::quick::quick_sort(&mut vec![5, 1, 60, 30]));
    // println!("Longest Consecutive Sequence: {:?}", problems::consecutive::longest_consecutive_sequence(&vec![1, 9, 3, 4, 10, 5, 2, 100]));
    // exercises::parallelism::spawn_threads(10);
    // println!("{}", exercises::parallelism::do_calc(&vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30]));
    // println!("{}", problems::simplify_path::simplify_path(&"/Users/Joma/Documents/..//Desktop/./..".to_string()));
    // println!("{}", problems::excel_columns::column_name(100000));
    // println!("{:?}", problems::subarray_sum::subarray_sum(&vec![1,2,3,4,5], 7));
    // println!("{}", graphs::bfs::are_connected(&graphs::bfs::some_graph(), &0, &4));

    println!("{:?}", graphs::bfs::shortest_path(&graphs::bfs::some_graph(), &5, &1))
}
