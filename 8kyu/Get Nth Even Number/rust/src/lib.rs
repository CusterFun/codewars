fn nth_even(n: u32) -> u32 {
    // unimplemented!();
    (n-1) * 2
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    
    #[test]
    fn example_tests() {
        assert_eq!(nth_even(1), 0);
        assert_eq!(nth_even(2), 2);
        assert_eq!(nth_even(3), 4);
        assert_eq!(nth_even(100), 198);
        assert_eq!(nth_even_best(1298734), 2597466);
    }

    #[test]
    fn random_tests() {
        let mut rnd = rand::thread_rng();
        let solution = |n: u32| { n * 2 - 2 };

        for _ in 0..100 {
            let n = rnd.gen_range(1..1000000000);

            assert_eq!(nth_even(n), solution(n));
        }
    }

    fn nth_even_best(n: u32) -> u32 {
        (0..).step_by(2).nth(n as usize - 1).unwrap()
    }
}