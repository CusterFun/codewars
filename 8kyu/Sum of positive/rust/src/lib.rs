fn positive_sum(slice: &[i32]) -> i32 {
    // 标记：fold 用法
    slice.iter().fold(0, |sum, &i| if i > 0 { sum + i } else { sum })
    // let mut sum = 0;
    // for i in slice.iter() {
    //     if *i > 0 { 
    //         sum += i;
    //     }
    // }
    // sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn some_examples() {
        assert_eq!(positive_sum(&[1,2,3,4,5]), 15);
        assert_eq!(positive_sum(&[1,-2,3,4,5]), 13);
        assert_eq!(positive_sum_filter(&[-1,2,3,4,-5]), 9);
    }
    
    #[test]
    fn empty_list() {
        assert_eq!(positive_sum(&[]), 0);
    }
    
    #[test]
    fn all_negative() {
        assert_eq!(positive_sum(&[-1,-2,-3,-4,-5]), 0);
    }
    
    use rand::Rng;
    
    #[test]
    fn random_arrays() {
        let mut rng = rand::thread_rng();
        
        for _ in 0..40 {
            let arr: Vec<i32> = (0..rng.gen_range(5..120))
                .map(|_| rng.gen_range(-100..100)).collect();
            let solution = arr.iter().filter(|i| **i > 0).sum();
            assert_eq!(positive_sum(&arr), solution);
        }
      
    }  
    
    // 标记：filter用法
    fn positive_sum_filter(arr: &[i32]) -> i32 {
        arr.iter().filter(|x| x.is_positive()).sum()
        arr.iter().filter(|&&x| x > 0).sum()
    }
}