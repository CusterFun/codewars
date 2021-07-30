pub fn quadratic(x1: i32, x2: i32) -> (i32, i32, i32) {
    (1, -x1 - x2, x1 * x2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn example_tests() {
        assert_eq!(quadratic(0, 1), (1, -1, 0));
        assert_eq!(quadratic(1, 1), (1, -2, 1));
        assert_eq!(quadratic(-4, -9), (1, 13, 36));
        assert_eq!(quadratic(-5, -4), (1, 9, 20));
        assert_eq!(quadratic(4, -9), (1, 5, -36));
        assert_eq!(quadratic(5, -4), (1, -1, -20));
    }

    fn solution(x1: i32, x2: i32) -> (i32, i32, i32) {
        (1, -(x1 + x2), x1 * x2)
    }

    #[test]
    fn fixed_tests() {
        assert_eq!(quadratic(4, 9), (1, -13, 36));
        assert_eq!(quadratic(7, 3), (1, -10, 21));
        assert_eq!(quadratic(43, 52), (1, -95, 2236));
        assert_eq!(quadratic(0, -1), (1, 1, 0));
        assert_eq!(quadratic(-3, -7), (1, 10, 21));
        assert_eq!(quadratic(-5, -4), (1, 9, 20));
        assert_eq!(quadratic(-19, -19), (1, 38, 361));
        assert_eq!(quadratic(-1, 1), (1, 0, -1));
        assert_eq!(quadratic(13, -108), (1, 95, -1404));
    }

    #[test]
    fn random_tests() {
        for _ in 0..10 {
            let mut rng = rand::thread_rng();
            let x1 = rng.gen_range(-100..100);
            let x2 = rng.gen_range(-100..100);
            assert_eq!(quadratic(x1, x2), solution(x1, x2));
        }
    }
}