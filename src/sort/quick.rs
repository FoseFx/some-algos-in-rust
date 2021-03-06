///
/// Quicksort is an algorithm that runs in n*log(n) (up to n^2).
/// It's not stable and recursive.
/// 
/// It sorts in-place, but also returns the sorted Vector.
/// 
/// Here's how to use it:
/// ```
/// use sort::quick;
/// ...
/// 
/// ```
/// 
pub fn quick_sort<T: std::cmp::Ord + std::clone::Clone>(vec: &mut Vec<T>) -> &Vec<T> {
    sort(vec, 0, vec.len() - 1);
    return vec;
}

///
/// The arguments mark the `left` and `right` ends of the part we look at.
/// They are indexes.
/// 
fn sort<T: std::cmp::Ord + std::clone::Clone>(vec: &mut Vec<T>, left: usize, right: usize) {
    let new_index: isize = handle_frame(vec, left, right) as isize;
    if (left as isize) < new_index - 1 {
        sort(vec, left, (new_index as usize) - 1);
    }
    if new_index < right as isize {
        sort(vec, new_index as usize, right);
    }
}

///
/// This function steps through the frame defined by `left` and `right` from both ends
/// comparing the values with the pivot defined by `right`
fn handle_frame<T: std::cmp::Ord + std::clone::Clone>(vec: &mut Vec<T>, left: usize, right: usize) -> usize {
    let mut i: usize = left;
    let mut j: usize = right;
    let piv = vec[(left + right) / 2].clone();
    while i <= j { // at the point where i and j meet, the frame was fuly handled
        while vec[i] < piv { // i >= right would mean we already went through everything, we look for sth bigger equal piv
            i += 1;
        }
        while vec[j] > piv { // j <= left  would mean we already went through everything, we look for sth smaller piv
            j -= 1;
        }
        if i <= j {
            swap(vec, i, j);
            i += 1;
            j -= 1;
        }
    }
    return i; // i will be in the middle of the frame
}

/// What do you expect this does?
fn swap<T: std::clone::Clone>(vec: &mut Vec<T>, a: usize, b: usize) {
    let m: T = vec[a].clone();
    vec[a] = vec[b].clone();
    vec[b] = m;
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn quick_sort_test() {
        let mut vec = vec![5, 7, 2, 7, 1, 8, 4, 9, 10];
        quick_sort(&mut vec);
        assert_eq!(vec, vec![1, 2, 4, 5, 7, 7, 8, 9, 10]);
    }
}