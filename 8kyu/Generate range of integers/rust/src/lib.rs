fn generate_range(min: usize, max: usize, step: usize) -> Vec<usize> {
    // unimplemented!();
    let mut res:Vec<usize> = Vec::new();
    let mut num = min;
    while num <= max {
        res.push(num);
        num += step;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{thread_rng, Rng};
    
    #[test]
    fn test_basic_cases() {
        assert_eq!(generate_range(2, 10, 2), [2,4,6,8,10]);
        assert_eq!(generate_range(1, 10, 3), [1,4,7,10]);
        assert_eq!(generate_range(1, 10, 1), [1,2,3,4,5,6,7,8,9,10]);
        assert_eq!(generate_range(1, 10, 4), [1,5,9]);
        assert_eq!(generate_range_best(1, 10, 5), [1,6]);
    }
    
    #[test]
    fn test_random_cases() {
        let mut rng = thread_rng();
        for _ in 1..40 {
            let min = rng.gen_range(0..20);
            let max = rng.gen_range(0..70) + 30;
            let step = rng.gen_range(1..10);
            assert_eq!(generate_range(min, max, step), answer(min, max, step));        
        }
    }

    fn answer(min: usize, max: usize, step: usize) -> Vec<usize> {
        let mut result = vec!();
        let mut index = min;
        while index <= max {
            result.push(index);
            index = index + step;
        }
        result
    }

    // 标记：step_by用法
    fn generate_range_best(min: usize, max: usize, step: usize) -> Vec<usize> {
        (min..=max).step_by(step).collect()
    }
}
