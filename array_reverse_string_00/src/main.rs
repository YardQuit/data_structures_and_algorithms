fn main() {
    let string: String = String::from("reverse");
    println!("original string: {}", string);
    match reverse_0(&string) {
        Some(reversed) => println!("reversed string: {}", reversed),
        None => println!("this cannot be reversed"),
    }
}

fn reverse_0(string: &str) -> Option<String> {
    if string.is_empty() {
        None
    } else {
    Some(string.chars().rev().collect())
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_reverse_0_success() {
        assert_eq!(reverse_0("reverse"), Some("esrever".to_owned()));
    }

    #[test]
    fn test_reverse_0_fail() {
        assert_eq!(reverse_0(""), None);
    }
}