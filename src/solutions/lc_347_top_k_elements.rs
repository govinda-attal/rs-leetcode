
pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    use std::collections::HashMap;
    
    let mut map: HashMap<i32, i32> = HashMap::new();

    for num in nums {
        *map.entry(num).or_insert(0)+=1;
    }
    
    let mut rs = map.into_iter().collect::<Vec<(i32,i32)>>();
    rs.sort_by(|a,b|b.1.cmp(&a.1));
    rs.truncate(k as usize);

    rs.into_iter().map(|(v, _)|v).collect::<_>()
}


#[cfg(test)]
mod test{
    use super::top_k_frequent;

    #[test]
    fn ex1() {
        assert_eq!(top_k_frequent(vec![1,1,1,2,2,3], 2), vec![1,2])
    }

    #[test]
    fn ex2() {
        assert_eq!(top_k_frequent(vec![1], 1), vec![1])
    }
}