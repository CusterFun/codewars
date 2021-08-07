fn reverse_seq(n: u32) -> Vec<u32> {
    // unimplemented!();
    (1..=n).rev().map(|i| i).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
  
    use std::time::{SystemTime, UNIX_EPOCH};

    fn sol(n: u32) -> Vec<u32> {
        (1..n + 1).rev().collect()
    }

    #[test]
    fn sample_test() {
        assert_eq!(reverse_seq(5), [5, 4, 3, 2, 1].to_vec());
    }

    #[test]
    fn basic_test() {
        assert_eq!(reverse_seq(6), [6, 5, 4, 3, 2, 1].to_vec());
        assert_eq!(reverse_seq(100), sol(100));
        assert_eq!(reverse_seq(10000), sol(10000));
        assert_eq!(reverse_seq(100000), sol(100000));
        assert_eq!(reverse_seq(1000000), sol(1000000));
    }

    // really basic system time-based rng since I don't trust Codewars
    #[test]
    fn random_test() {
        let a = 1140671485;
        let c = 128201163;
        let m = 2 << 24;
        let mut seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        for _ in 0..100 {
            seed = (a * seed + c) % m;
            let n = (seed % 10000 + 2) as u32;
            assert_eq!(reverse_seq(n), sol(n));
        }
    }
}