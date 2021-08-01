fn summation(n: i32) -> i32 {
    // unimplemented!();
    (1..=n).collect::<Vec<i32>>().iter().sum()
}

#[cfg(test)]
mod tests {
    use super::summation;
    use rand::Rng;

    #[test]
    fn basic_tests() {
        assert_eq!(summation(1), 1);
        assert_eq!(summation_sum(8), 36);
        assert_eq!(summation(22), 253);
        assert_eq!(summation_fold(100), 5050);
        assert_eq!(summation(213), 22791);
    }
    
    fn sol(n: i32) -> i32 {
        n * (n + 1) / 2
    }
    
    #[test]
    fn random_tests() {
        let mut r = rand::thread_rng();
        for _ in 0..100 {
            let n = r.gen_range(1..500);
            let expected = sol(n);
            assert_eq!(summation(n), expected);
        }
    }

    // 标记：sum用法
    fn summation_sum(n: i32) -> i32 {
        (1..=n).sum()
    }
    // 标记：fold用法
    fn summation_fold(n: i32) -> i32 {
        (1..=n).fold(0, |acc, val| acc + val)
    }
}