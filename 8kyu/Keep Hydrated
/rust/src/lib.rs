fn litres(time: f64) -> i32 {
    time as i32/ 2
}

#[cfg(test)]
mod tests {
    use super::litres;

    #[test]
    fn sample_tests() {
        assert_eq!(litres(2.), 1);
        assert_eq!(litres(1.4), 0);
        assert_eq!(litres(12.3), 6);
        assert_eq!(litres(0.82), 0);
        assert_eq!(litres(11.8), 5);
        assert_eq!(litres(1787.), 893);
        assert_eq!(litres(0.), 0);
    }
    
    #[test]
    fn random_tests() {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let time = rng.gen_range(0.0..1000.0);
            let expected = (time / 2f64) as i32;
            assert_eq!(litres(time), expected);
        }
    }
}