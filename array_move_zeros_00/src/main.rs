fn main() {
    let array_1 = vec![0, 1, 0, 3, 12];
    let array_2 = move_zeros(array_1).unwrap(); 
    println!("{:?}", array_2);
}

fn move_zeros(mut array: Vec<i32>) -> Option<Vec<i32>> {
    if array.is_empty() {
        return None;
    }

    let len = array.len();
    for i in 0..len {
        if array[i] == 0 {
            array.push(array[i]);
            array.remove(i);
        }
    }
    Some(array)
}

/*
    TEST SECTION BEGINS
*/

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn move_zeros_failure() {
        let array: Vec<i32> = Vec::new();
        assert_eq!(move_zeros(array), None);
    }
    #[test]
    fn move_zeros_success() {
        let array = vec![0, 1, 0, 3, 12];
        assert_eq!(move_zeros(array).unwrap(), [1, 3, 12, 0, 0]);
    }
}