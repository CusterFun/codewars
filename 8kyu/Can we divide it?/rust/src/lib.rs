fn is_divide_by(number: i32, a: i32, b: i32) -> bool {
    number % a == 0 && number % b == 0
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html
fn solution(number: i32, a: i32, b: i32) -> bool {
    number % a == 0 && number % b == 0
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{distributions::Uniform, prelude::*};

    #[test]
    fn basic_tests() {
        assert_eq!(is_divide_by(8, 2, 4), true);
        assert_eq!(is_divide_by(12, -3, 4), true);
        assert_eq!(is_divide_by(8, 3, 4), false);
        assert_eq!(is_divide_by(48, 2, -5), false);
        assert_eq!(is_divide_by(-100, -25, 10), true);
        assert_eq!(is_divide_by(10000, 5, -3), false);
        assert_eq!(is_divide_by(4, 4, 2), true);
        assert_eq!(is_divide_by(5, 2, 3), false);
        assert_eq!(is_divide_by(-96, 25, 17), false);
        assert_eq!(is_divide_by(33, 1, 33), true);
    }

    #[test]
    fn random_tests() {
        let mut rng = thread_rng();
        let a_distr = Uniform::from(1..=4);
        let b_distr = Uniform::from(1..=7);
        let exp_distr = Uniform::from(1..=3);

        for _ in 0..20 {
            let n_max: i32 = 10i32.overflowing_pow(rng.sample(exp_distr)).0;
            let n: i32 = rng.gen_range(1..=n_max);
            let a: i32 = rng.sample(a_distr);
            let b: i32 = rng.sample(b_distr);

            assert_eq!(is_divide_by(n, a, b), solution(n, a, b));
        }
    }
}