
/// Given an array arr and a number k
/// this function returns an Option may containing a
/// subarray of arr with elements that sum to k.
pub fn subarray_sum(arr: &Vec<usize>, k: usize) -> Option<Vec<usize>> {

    if arr.is_empty() { // no subarrays possible
        return None;
    }

    let mut lower = 0;
    let mut sum = arr[0]; // must exist

    for higher in 1..=arr.len() { // i <= arr.len
        while sum > k && lower < higher - 1 { // sum too high? up lowest
            sum -= arr[lower];
            lower += 1;
        }
        if sum == k {
            return Some(arr[lower..higher].to_vec());
        }
        if higher < arr.len() {
            sum += arr[higher]; // add this element to sum
        }

    }

    return None;
}


#[cfg(test)]
mod tests {
    use super::*;

    fn sum_of_vec(vec: &Vec<usize>) -> usize {
        return vec.iter().fold(0, |x, &v| x + v);
    }

    #[test]
    fn test_sum_of_vec() {
        assert_eq!(sum_of_vec(&vec![1,2,3,4,5]), 15);
    }

    #[test]
    fn test_subarray_sum() {

        // Empty arr
        let mut k = 12;
        let mut arr = vec![];
        let mut res = subarray_sum(&arr, k);
        assert_eq!(res, None);

        // k not summable
        k = 5;
        arr = vec![7, 8, 9];
        res = subarray_sum(&arr, k);
        assert_eq!(res, None);

        // "normal" case
        k = 9;
        arr = vec![7, 8, 9, 3, 4, 2, 1, 0];
        res = subarray_sum(&arr, k);
        assert_ne!(res, None);
        assert_eq!(sum_of_vec(&res.unwrap()), k);


    }
}