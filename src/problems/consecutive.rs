use std::collections::HashSet;

///
/// Given a sequence of numbers in any order (Vec<usize>)
/// this function returns the longest sequence of numbers
/// in order
///
/// Example:
/// For
///     [ 9, 3, 4, 10, 5, 2, 100 ]
///  it returns
///     [ 2, 3, 4, 5 ]
///
pub fn longest_consecutive_sequence(sequence: &Vec<usize>) -> Vec<usize> {
    let mut lookup_set: HashSet<usize> = HashSet::new();

    // Step 1: Insert all elements into lookup_set
    for number in sequence {
        lookup_set.insert(*number);
    }

    // Step 2: Find lcs
    let mut lcs: usize = 0; // the length of the longest sub sequence
    let mut lcs_max = 0; // the upper-limit of it (inclusive)
    let mut lcs_min = 0; // the lower-limit of it (inclusive)
    for number in sequence {
        let number_below = *number - 1;
        // if number_below does not exists in lookup_set, number begins a new sequence
        // else number is part of one, and we move on to the next
        if !lookup_set.contains(&number_below) {
            let eos = find_end_of_sequence(&lookup_set, *number);
            let len = eos - *number + 1;
            if len > lcs { // new sequence is longer than old one
                lcs = len;
                lcs_max = eos;
                lcs_min = *number;
            }
        }
    }

    // Return Sequence
    let mut ret: Vec<usize> = vec![];
    for n in lcs_min..=lcs_max { // iterate from lcs_min to lcs_max
        ret.push(n);
    }
    return ret;
}

fn find_end_of_sequence(lookup_set: &HashSet<usize>, start_of_sequence: usize) -> usize {
    let mut eos = start_of_sequence;
    loop {
        let next = eos + 1;
        if !lookup_set.contains(&next) {
            // next is not in set anymore, eos found
            break;
        }
        eos = next;
    }
    return eos;
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_longest_consecutive_sequence() {
        assert_eq!(longest_consecutive_sequence(&vec![9,3,4,10,5,2,100]), [2,3,4,5]);
        assert_eq!(longest_consecutive_sequence(&vec![1,2,3,6,7,8,9]), [6,7,8,9]);
    }
}