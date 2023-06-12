pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let (mut i1, mut i2, l1, l2) = (0 as usize, 0 as usize, nums1.len(), nums2.len());
    let mut merged_list = vec![];

    if l1 == 0 && l2 == 0 {
        return 0 as f64;
    }

    while i1 < l1 && i2 < l2 {
        while i1 < l1 && i2 < l2 && nums1[i1] <= nums2[i2] {
            merged_list.push(nums1[i1]);
            i1 += 1;
        }
        while i1 < l1 && i2 < l2 && nums2[i2] <= nums1[i1] {
            merged_list.push(nums2[i2]);
            i2 += 1;
        }
    }

    merged_list.extend(nums1[i1..].iter());
    merged_list.extend(nums2[i2..].iter());

    let mid = merged_list.len() / 2;
    if merged_list.len() % 2 == 1 {
        return merged_list[mid] as f64;
    }
    (merged_list[mid] + merged_list[mid - 1]) as f64 / 2 as f64
}

#[cfg(test)]
mod test {
    use crate::solutions::lc_004_median_sorted_slices::find_median_sorted_arrays;

    #[test]
    fn test_ex1() {
        let nums1 = vec![];
        let nums2 = vec![];
        assert_eq!(find_median_sorted_arrays(nums1, nums2), 0 as f64)
    }

    #[test]
    fn test_ex2() {
        let nums1 = vec![1];
        let nums2 = vec![];
        assert_eq!(find_median_sorted_arrays(nums1, nums2), 1 as f64)
    }

    #[test]
    fn test_ex3() {
        let nums1 = vec![1, 3];
        let nums2 = vec![2];
        assert_eq!(find_median_sorted_arrays(nums1, nums2), 2 as f64)
    }

    #[test]
    fn test_ex4() {
        let nums1 = vec![1, 2];
        let nums2 = vec![3, 4];
        assert_eq!(find_median_sorted_arrays(nums1, nums2), 2.5 as f64)
    }

    #[test]
    fn test_ex5() {
        let nums1 = vec![100000];
        let nums2 = vec![100001];
        assert_eq!(find_median_sorted_arrays(nums1, nums2), 100000.5 as f64)
    }
}
