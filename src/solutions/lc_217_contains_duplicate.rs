use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let uniques = nums.iter().map(|x|*x).collect::<HashSet<i32>>();
    return uniques.len() != nums.len()
}

#[cfg(test)]
mod test{
    use crate::solutions::lc_217_contains_duplicate::contains_duplicate;

    #[test]
    fn ex1() {
        assert!(contains_duplicate(vec![1,2,3,1]));
    }

    #[test]
    fn ex2() {
        assert!(!contains_duplicate(vec![1,2,3,4]));
    }

    #[test]
    fn ex3() {
        assert!(contains_duplicate(vec![1,1,1,3,3,4,3,2,4,2]));
    }

}

