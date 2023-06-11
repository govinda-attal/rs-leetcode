pub fn is_palindrome(phrase: &str) -> bool {
    let val = phrase
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
        .to_lowercase();

    return val == val.chars().rev().collect::<String>();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert!(is_palindrome("pap"))
    }
}
