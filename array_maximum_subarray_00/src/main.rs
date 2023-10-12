fn main() {
    let array = vec![-1, 1, -3, 4, -1, 2, 1, -5, 4];
    let maximum = maximum_subarray(array).unwrap();
    println!("{}", maximum);
}

fn maximum_subarray(array: Vec<i32>) -> Option<i32> {
    if array.is_empty() {
        return None;
    }

    let mut maximum_sum = array[0];
    let mut current_sum = array[0];

    let len = array.len();
    for i in 1..len {
        current_sum = i32::max(array[i], current_sum + array[i]);
        maximum_sum = i32::max(maximum_sum, current_sum);
    }
    Some(maximum_sum)
}

/*
    TEST SECTION BEGINS
*/

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn maximum_subarray_failure() {
        let array: Vec<i32> = Vec::new();
        assert_eq!(maximum_subarray(array), None);
    }
    #[test]
    fn maximum_subarray_success() {
        let array = vec![-1, 1, -3, 4, -1, 2, 1, -5, 4];
        assert_eq!(maximum_subarray(array).unwrap(), 6);
    }
}