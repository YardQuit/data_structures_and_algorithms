const KEY: usize = 3;

fn main() {
    let array_1 = vec![1, 2, 3, 4, 5, 6, 7];
    let array_2 = rotate_array(array_1, KEY);
    match array_2 {
        Some(rotated) => println!("{:?}", rotated),
        None => println!("failed to rotate array"),
    }
}

fn rotate_array<T: Clone>(array: Vec<T>, key: usize) -> Option<Vec<T>> {
    if array.is_empty() {
        return None;
    }

    let len = array.len();
    let mut rotated = Vec::new();

    for i in 0..len {
        let shift: usize = (key + i + 1) % len;
        rotated.push(array[shift].clone());
    }
    Some(rotated)
}

/*
    TESTING SECTION BEGINS
*/

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn rotate_array_failure() {
        let array: Vec<i32> = Vec::new();
        assert_eq!(rotate_array(array, 3), None);
    }
    #[test]
    fn rotate_array_three_steps_to_the_right_start_with_fourth_success() {
        let array = vec![1, 2, 3, 4, 5, 6, 7];
        assert_eq!(rotate_array(array, 3).unwrap(), [5, 6, 7, 1, 2, 3, 4]);
        
    }
}