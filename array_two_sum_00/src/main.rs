use std::collections::HashMap;

const TARGET: usize = 9;

fn main() {
    let nums: Vec<usize> = vec![2, 7, 11, 15];
    let combo = find_combo(&nums, &TARGET);

    match combo {
        Some(index) => println!("{:?}", index),
        None => println!("could not find combo"),
    }
}

fn find_combo(nums: &Vec<usize>, target: &usize) -> Option<(usize, usize)> {
    if nums.is_empty() {
        return None;
    }

    let mut num_index = HashMap::new();

    for (index, &num) in nums.iter().enumerate() {
        num_index.insert(num, index);
    }

    for (index, &num) in nums.iter().enumerate() {
        let complement = target - num;

        if let Some(&complement_index) = num_index.get(&complement) {
            if index != complement_index {
                return Some((index, complement_index));
            }
        }
    }
    None
}

/*
    TEST SECTION BEGIN
*/

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn two_sum_empty_array_failure() {
        const TARGET: usize = 9;
        let nums: Vec<usize> = Vec::new();
        assert_eq!(find_combo(&nums, &TARGET), None);
        
    }
    #[test]
    fn two_sum_failure() {
        const TARGET: usize = 10;
        let nums: Vec<usize> = vec![2, 7, 11, 15];
        assert_eq!(find_combo(&nums, &TARGET), None);
        
    }
    #[test]
    fn two_sum_success() {
        const TARGET: usize = 9;
        let nums: Vec<usize> = vec![2, 7, 11, 15];
        assert_eq!(find_combo(&nums, &TARGET).unwrap(), (0, 1));
    }
}