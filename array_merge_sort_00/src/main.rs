fn main() {
    let array_1 = vec!['a', 'h', 'c', 'e'];
    let array_2 = vec!['b', 'g', 'd', 'f'];
    let array_3 = combind_arrays(array_1, array_2).unwrap();
    let array_4 = divide_and_conquer_array(array_3);
    match array_4 {
        Some(array) => {
            for element in array {
                print!("{} ", element);
            }
            println!();
        }
        None => println!("no array available"),
    }
}

fn combind_arrays<T: Ord + Clone>(array_left: Vec<T>, array_right: Vec<T>) -> Option<Vec<T>> {
    if array_left.is_empty() && array_right.is_empty() {
        None
    } else if !array_left.is_empty() && array_right.is_empty() {
        return Some(array_left);
    } else if array_left.is_empty() && !array_right.is_empty() {
        return Some(array_right);
    } else {
        let mut array = Vec::new();
        array.extend(array_left);
        array.extend(array_right);
        Some(array)
    }
}

fn divide_and_conquer_array<T: Ord + Clone>(array: Vec<T>) -> Option<Vec<T>> {
    if array.is_empty() {
        return None;
    }

    let len = array.len();
    if len <= 1 {
        return Some(array);
    }

    let mid = len / 2;
    let left = &array[..mid];
    let right = &array[mid..];

    let sorted_left = divide_and_conquer_array(left.to_vec());
    let sorted_right = divide_and_conquer_array(right.to_vec());

    merge_sort_arrays(sorted_left?, sorted_right?)
}

fn merge_sort_arrays<T: Ord + Clone>(mut left: Vec<T>, mut right: Vec<T>) -> Option<Vec<T>> {
    if left.is_empty() && right.is_empty() {
        return None;
    }

    let mut array = Vec::new();
    while !left.is_empty() && !right.is_empty() {
        if left[0] <= right[0] {
            array.push(left.remove(0));
        } else {
            array.push(right.remove(0));
        }
    }
    array.extend(left);
    array.extend(right);
    Some(array)
}

/*
    TEST SECTION BEGINS 
*/

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn divide_and_conqure_array_success() {
        let array = vec!['a', 'h', 'c', 'e', 'b', 'g', 'd', 'f'];
        assert_eq!(
            divide_and_conquer_array(array).unwrap(),
            ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h']
        )
    }
    #[test]
    fn merge_sort_arrays_two_empty_arrays_failure() {
        let array_1: Vec<char> = Vec::new();
        let array_2: Vec<char> = Vec::new();
        assert_eq!(merge_sort_arrays(array_1, array_2), None);
    }
    #[test]
    fn merge_sort_arrays_two_sorted_arrays_success() {
        let array_1 = vec!['a', 'b', 'c', 'd'];
        let array_2 = vec!['e', 'f', 'g', 'h'];
        assert_eq!(
            merge_sort_arrays(array_1, array_2).unwrap(),
            ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h']
        );
    }
    #[test]
    fn test_sort_arrays_left_empty_one_sorted_success() {
        let array_1: Vec<char> = Vec::new();
        let array_2 = vec!['e', 'f', 'g', 'h'];
        assert_eq!(
            combind_arrays(array_1, array_2).unwrap(),
            ['e', 'f', 'g', 'h']
        );
    }
    #[test]
    fn test_sort_arrays_right_empty_one_sorted_success() {
        let array_1 = vec!['a', 'h', 'c', 'e'];
        let array_2: Vec<char> = Vec::new();
        assert_eq!(
            combind_arrays(array_1, array_2).unwrap(),
            ['a', 'h', 'c', 'e']
        );
    }
    #[test]
    fn test_combind_arrays_empty_faulure() {
        let array_1: Vec<char> = Vec::new();
        let array_2: Vec<char> = Vec::new();
        assert_eq!(combind_arrays(array_1, array_2), None);
    }
    #[test]
    fn test_combind_arrays_success() {
        let array_1 = vec!['a', 'h', 'c', 'e'];
        let array_2 = vec!['b', 'g', 'd', 'f'];
        assert_eq!(
            combind_arrays(array_1, array_2).unwrap(),
            ['a', 'h', 'c', 'e', 'b', 'g', 'd', 'f']
        );
    }
    #[test]
    fn test_combined_arrays_left_empty_success() {
        let array_1: Vec<char> = Vec::new();
        let array_2 = vec!['b', 'g', 'd', 'f'];
        assert_eq!(
            combind_arrays(array_1, array_2).unwrap(),
            ['b', 'g', 'd', 'f']
        );
    }
    #[test]
    fn test_combind_arrays_right_empty_success() {
        let array_1 = vec!['a', 'h', 'c', 'e'];
        let array_2: Vec<char> = Vec::new();
        assert_eq!(
            combind_arrays(array_1, array_2).unwrap(),
            ['a', 'h', 'c', 'e']
        );
    }
}
