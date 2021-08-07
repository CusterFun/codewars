fn double_integer(n: i32) -> i32 {
    n * 2
}

// https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html
#[cfg(test)]
mod tests {
    use super::double_integer;

    #[test]
    fn sample_tests() {
        assert_eq!(double_integer(1), 2);
        assert_eq!(double_integer(5), 10);
        assert_eq!(double_integer(10), 20);
        assert_eq!(double_integer(20), 40);
        assert_eq!(double_integer(40), 80);
    }
    
    #[test]
    fn random_tests() {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let test_q = 256;
        
        for _ in 0..test_q {
            let input = rng.gen_range(0..i32::MAX / 3);
            let expected = 2 * input;
            assert_eq!(double_integer(input), expected);
            assert_eq!(double_integer(-input), -expected);
        }
    }
}