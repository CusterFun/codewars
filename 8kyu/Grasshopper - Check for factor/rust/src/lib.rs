fn check_for_factor(base: i32, factor: i32) -> bool {
    base % factor == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(check_for_factor(10, 2), true);
        assert_eq!(check_for_factor(63, 7), true);
        assert_eq!(check_for_factor(2450, 5), true);
        assert_eq!(check_for_factor(24612, 3), true);
        assert_eq!(check_for_factor(9, 2), false);
        assert_eq!(check_for_factor(653, 7), false);
        assert_eq!(check_for_factor(2453, 5), false);
        assert_eq!(check_for_factor(24617, 3), false);
    }

    fn sol(b: i32, f: i32) -> bool {
        b % f == 0
    }
    
    #[test]
    fn random_tests() {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let b = rng.gen_range(20..10000);
            let f = rng.gen_range(1..20);
            assert_eq!(check_for_factor(b, f), sol(b, f));
        }
    }
}