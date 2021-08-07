fn next_id(ids: &[usize]) -> usize {
    // 标记：find 、contains用法
    (0..).find(|n| !ids.contains(&n)).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    use rand::{thread_rng, Rng};
    use rand::seq::SliceRandom;

    #[test]
    fn test_basics() {
        assert_eq!(next_id(&[0,1,2,4,5]), 3);
        assert_eq!(next_id(&[0,1,2,3,4,5,6,7,8,9,10]), 11);
        assert_eq!(next_id(&[]), 0);
    }
    
    #[test]
    fn test_unsorted() {
        assert_eq!(next_id(&[1,2,0,3]), 4);
        assert_eq!(next_id(&[1,0,2,5,3]), 4);
    }
    
    #[test]
    fn test_unsorted_duplicates() {
        assert_eq!(next_id(&[1,2,0,2,3]), 4);
        assert_eq!(next_id(&[1,2,0,2,3,5]), 4);
    }
    
    #[test]
    fn test_random() {
        let mut rng = thread_rng();
        (0..50).for_each(|count| {
            let id: usize = rng.gen_range(0..=count);
            // 标记：filter 用法
            let mut ids: Vec<usize> = (0..=count).filter(|num| num != &id).collect();
            ids.shuffle(&mut rng);
            assert_eq!(next_id(&ids), id, "failed with {:?}", ids);
        });
    }
}