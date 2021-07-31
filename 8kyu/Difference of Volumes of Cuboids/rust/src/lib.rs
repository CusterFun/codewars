fn find_difference(a: &[i32; 3], b: &[i32; 3]) -> i32 {
    (a.iter().fold(1, |acc, x| acc * x) - b.iter().fold(1, |acc, x| acc * x)).abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(find_difference(&[3, 2, 5], &[1, 4, 4]), 14);
        assert_eq!(find_difference(&[9, 7, 2], &[5, 2, 2]), 106);
        assert_eq!(find_difference(&[11, 2, 5], &[1, 10, 8]), 30);
        assert_eq!(find_difference(&[4, 4, 7], &[3, 9, 3]), 31);
        assert_eq!(find_difference_best(&[15, 20, 25], &[10, 30, 25]), 0);
    }
    
    use rand::{thread_rng, Rng};
    
    #[test]
    fn random() {
        let mut rng = thread_rng();
        
        for _ in 0..100 {
            let a = [rng.gen_range(1..=30), rng.gen_range(1..=30), rng.gen_range(1..=30)];
            let b = [rng.gen_range(1..=30), rng.gen_range(1..=30), rng.gen_range(1..=30)];
            
            let expect = find_difference_solution(&a, &b);
            
            assert_eq!(
                find_difference(&a, &b),
                expect,
                "a: {:?} b: {:?} should equal {}",
                &a, &b, expect,
            );
        }
    }
    
    fn find_difference_solution(a: &[i32], b: &[i32]) -> i32 {
        ((a[0] * a[1] * a[2]) - (b[0] * b[1] * b[2])).abs()
    }

    fn find_difference_best(a: &[i32], b: &[i32]) -> i32 {
        i32::abs(a.iter().product::<i32>() - b.iter().product::<i32>())
    }
}