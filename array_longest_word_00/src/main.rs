fn main() {
    let array_words = vec!["anna", "karin", "lars", "peter"];
    let array_longest = longest_words(array_words).unwrap();
    println!("{:?}", array_longest);
}

fn longest_words(words: Vec<&str>) -> Option<Vec<&str>> {
    if words.is_empty() {
        return None
    }

    let mut longest_words = Vec::new();
    let mut longest_seen = 0;

    for word in words {
        let word_len = word.len();
        use std::cmp::Ordering;

        match word_len.cmp(&longest_seen) {
            Ordering::Greater => {
                longest_seen = word_len;
                longest_words.clear();
                longest_words.push(word);
            }
            Ordering::Equal => {
                longest_words.push(word);
            }
            Ordering::Less => {}
        }
    }
    Some(longest_words)
}

/*
    TEST SECTION BEGINS
*/

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn longest_word_failure() {
        let array: Vec<&str> = Vec::new();
        assert_eq!(longest_words(array), None);
    }
    #[test]
    fn longest_word_success() {
        let array = vec!["a", "ab", "abc", "b", "bcb", "b"];
        assert_eq!(longest_words(array).unwrap(), ["abc", "bcb"]);
    }
}