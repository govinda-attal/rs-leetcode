struct Solution {
    bad_version: i32,
}

impl Solution {
    pub fn new(bv: i32) -> Self {
        Self { bad_version: bv }
    }

    fn is_bad_version(&self, v: i32) -> bool {
        self.bad_version <= v
    }

    pub fn get_first_bad_version(&self, n: i32) -> i32 {
        let mut good = 1;
        let mut bad = n;
        loop {
            let mid = ((bad as i64 + good as i64) / 2) as i32;
            if self.is_bad_version(mid) {
                bad = mid
            } else {
                good = mid
            }
            if self.is_bad_version(good) {
                bad = good;
                break;
            }
            if good + 1 == bad {
                break;
            }
            if good == bad {
                break;
            }
        }
        bad
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    #[test]
    fn test_bad_version() {
        let sol = Solution::new(1702766719);
        let bv = sol.get_first_bad_version(2126753390);
        assert_eq!(bv, 1702766719)
    }

    #[test]
    fn test_first() {
        let sol = Solution::new(1);
        let bv = sol.get_first_bad_version(1);
        assert_eq!(bv, 1)
    }
}
