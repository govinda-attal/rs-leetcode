use std::collections::HashMap;


fn reduce(cur: char, prev: char) -> bool{
    if cur == 'I' && prev == 'V' {
        true
    } else if cur== 'I' && prev == 'X' {
        true
    } else if cur== 'X' && prev == 'L' {
        true
    } else if cur== 'X' && prev == 'C' {
        true
    } else if cur== 'C' && prev == 'D' {
        true
    } else if cur== 'C' && prev == 'M' {
        true
    } else {
        false
    }
}

pub fn roman_to_int(s: String) -> i32 {
    let values = HashMap::from([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]);
    let mut prev = ' ';
    let mut result = 0;
    for cur in s.chars().rev() {
        let v = values.get(&cur).unwrap();
        if reduce(cur, prev){
            result -= v;
        }else {
            result += v;
        }
        prev = cur;
    }
    result
}




#[cfg(test)]
mod test{
    use super::roman_to_int;


    #[test]
    fn ex1() {
        assert_eq!(roman_to_int("III".to_string()), 3)
    }

    #[test]
    fn ex2() {
        assert_eq!(roman_to_int("LVIII".to_string()), 58)
    }
    
    #[test]
    fn ex3() {
        assert_eq!(roman_to_int("MCMXCIV".to_string()), 1994)
    }
}