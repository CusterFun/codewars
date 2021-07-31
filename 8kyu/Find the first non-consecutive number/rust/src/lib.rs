fn first_non_consecutive(arr: &Vec<i32>) -> Option<i32> {
    // code here
    for i in 1..arr.len() {
        if arr[i] - arr[i-1] != 1 {
            return Some(arr[i]);
        }
    }
    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(first_non_consecutive(&vec![1,2,3,4,6,7,8]), Some(6));
        assert_eq!(first_non_consecutive(&vec![1,2,3,4,5,6,7,8]), None);
        assert_eq!(first_non_consecutive(&vec![4,6,7,8,9,11]), Some(6));
        assert_eq!(first_non_consecutive(&vec![4,5,6,7,8,9,11]), Some(11));
        assert_eq!(first_non_consecutive(&vec![31,32]), None);
        assert_eq!(first_non_consecutive(&vec![-3,-2,0,1]), Some(0));
        assert_eq!(first_non_consecutive_best(&vec![-5,-4,-3,-1]), Some(-1));
    }
    
    fn sol(arr: &Vec<i32>) -> Option<i32> {
      for i in 1..arr.len() {
        if arr[i] - arr[i - 1] != 1 {
          return Some(arr[i]);
        }
      };
      None
    }
    
    use rand::{thread_rng, Rng};

    #[test]
    fn test_random() {
        let mut rng = thread_rng();
        
        for _ in 0..40 {
            let mut arr: Vec<i32> = (rng.gen_range(1..6)..=rng.gen_range(7..12)).collect();
            if rng.gen_bool(0.5) {
              arr.remove(rng.gen_range(0..arr.len()));
            };
            let ar: Vec<i32> = arr;
            assert_eq!(first_non_consecutive(&ar), sol(&ar));
        };
    }

    // 标记：windows，find，map用法
    fn first_non_consecutive_best(arr: &[i32]) -> Option<i32> {
        arr.windows(2).find(|s| s[0] + 1 != s[1]).map(|s| s[1])
    }
}
