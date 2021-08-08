fn other_angle(a: u32, b: u32) -> u32 {
    180 - a - b
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn example_tests() {
        assert_eq!(other_angle(30, 60), 90);
        assert_eq!(other_angle(60, 60), 60);
        assert_eq!(other_angle(43, 78), 59);
        assert_eq!(other_angle(10, 20), 150);
    }
    
    #[test]
    fn random_tests() {
        fn _other_angle(a: u32, b: u32) -> u32 {
            180 - a - b
        }
        
        let mut rng = rand::thread_rng();
        
        for _ in 0..100 {
            let a = rng.gen_range(1..179);
            let b = rng.gen_range(1..180 - a);
            let c = _other_angle(a, b);
            assert_eq!(other_angle(a, b), c);
        }
    }
}
