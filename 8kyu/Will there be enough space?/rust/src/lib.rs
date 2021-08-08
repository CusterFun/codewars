fn enough(cap: i32, on: i32, wait: i32) -> i32 {
    if cap - on - wait > 0 {
        return 0;
    }
    return wait - (cap - on);
}

#[cfg(test)]
mod tests {
    use super::enough;
    use rand::Rng;

    #[test]
    fn test_enough() {
        assert_eq!(enough(10, 5, 5), 0, "enough(10, 5, 5) should return 0");
        assert_eq!(enough(100, 60, 50), 10, "enough(100, 60, 50) should return 10");
        assert_eq!(enough(20, 5, 5), 0, "enough(20, 5, 5) should return 0");
    }
    
    #[test]
    fn test_enough_edge_cases() {
        assert_eq!(enough(0, 0, 0), 0, "enough(0, 0, 0) should return 0");
        assert_eq!(enough(1, 1, 1), 1, "enough(1, 1, 1) should return 1");
        assert_eq!(enough(1, 0, 1), 0, "enough(1, 0, 1) should return 0");
        assert_eq!(enough(1, 1, 0), 0, "enough(1, 1, 0) should return 0");
    }

    #[test]
    fn test_enough_random() {
        let mut rng = rand::thread_rng();
        
        for _ in 0..100 {
            let on = rng.gen_range(0..10);
            let wait = rng.gen_range(0..100);
            let cap = on + rng.gen_range(0..50);
            let expected = 0.max(on + wait - cap);
            assert_eq!(
                enough(cap, on, wait),
                expected,
                "enough({}, {}, {}) should return {}",
                cap,
                on,
                wait,
                expected
            );
        }
    }
}
