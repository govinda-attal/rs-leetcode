pub fn climb_stairs(n: i32) -> i32 {
    let mut prior_prev;
    let mut prior = 0;
    let mut curr = 1;

    for _ in 1..=n {
        prior_prev = prior;
        prior = curr;
        curr = prior + prior_prev;
    }
    curr
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(climb_stairs(1), 1)
    }
    #[test]
    fn test_2() {
        assert_eq!(climb_stairs(2), 2)
    }
    #[test]
    fn test_3() {
        assert_eq!(climb_stairs(3), 3)
    }
    #[test]
    fn test_4() {
        assert_eq!(climb_stairs(4), 5)
    }
    #[test]
    fn test_5() {
        assert_eq!(climb_stairs(5), 8)
    }

    #[test]
    fn test_44() {
        assert_eq!(climb_stairs(44), 1134903170)
    }
}
