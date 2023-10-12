use std::collections::HashSet;

fn main() {
    let array = vec![1, 2, 3, 4, 5, 1];
    println!("has duplicates: {}", contains_duplicates(array));
}

fn contains_duplicates<T>(array: Vec<T>) -> bool 
where T: Copy + PartialEq + std::hash::Hash + std::cmp::Eq, {
    if array.is_empty() {
        return false;
    }

    let mut have_seen: HashSet<T> = HashSet::new();

    for &element in array.iter() {
        if have_seen.contains(&element) {
            return true;
        } else {
            have_seen.insert(element);
        }
    }
    false
}

/*
    TEST SECTION BEGINS
*/

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_contains_duplicates_empty_array_success() {
        let array: Vec<i32> = Vec::new();
        assert!(!contains_duplicates(array));
    }
    #[test]
    fn test_contains_duplicates_false_success() {
        let array = vec![1, 2, 3, 4, 5];
        assert!(!contains_duplicates(array));
    }
    #[test]
    fn test_contains_duplicates_true_success() {
        let array = vec![1, 2, 3, 3, 1];
        assert!(contains_duplicates(array));
    }
}