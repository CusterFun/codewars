fn expressions_matter(a: u64, b: u64, c: u64) -> u64 {
    // Your Code here... Happy Coding!
    *[a + b + c, a * (b + c), (a + b) * c, a * b * c].iter().max().unwrap()
}

use rand::Rng;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(expressions_matter(2, 1, 2), 6);
        assert_eq!(expressions_matter(1, 1, 1), 3);
        assert_eq!(expressions_matter(2, 2, 4), 16);
        assert_eq!(expressions_matter(3, 3, 3), 27);
        assert_eq!(expressions_matter(2, 1, 1), 4);
        assert_eq!(expressions_matter(1, 2, 3), 9);
        assert_eq!(expressions_matter(1, 3, 1), 5);
        assert_eq!(expressions_matter(2, 2, 2), 8);

        assert_eq!(expressions_matter(5, 1, 3), 20);
        assert_eq!(expressions_matter(3, 5, 7), 105);
        assert_eq!(expressions_matter(5, 6, 1), 35);
        assert_eq!(expressions_matter(1, 6, 1), 8);
        assert_eq!(expressions_matter(2, 6, 1), 14);
        assert_eq!(expressions_matter(6, 7, 1), 48);

        assert_eq!(expressions_matter(2, 10, 3), 60);
        assert_eq!(expressions_matter(1, 8, 3), 27);
        assert_eq!(expressions_matter(9, 7, 2), 126);
        assert_eq!(expressions_matter(1, 1, 10), 20);
        assert_eq!(expressions_matter(9, 1, 1), 18);
        assert_eq!(expressions_matter(10, 5, 6), 300);
        assert_eq!(expressions_matter(1, 10, 1), 12);
    }
    
    #[test]
    fn random_tests() {
        for _ in 0..100 {
            let a = rand::thread_rng().gen_range(1..11);
            let b = rand::thread_rng().gen_range(1..11);
            let c = rand::thread_rng().gen_range(1..11);
            let expected = expressions_matter_solution(a, b, c);
            assert_eq!(expressions_matter(a, b, c), expected);
        }
    }

    fn expressions_matter_solution(a: u64, b: u64, c: u64) -> u64 {
        let nums = [a * b * c, a + b + c, (a + b) * c, a * (b + c)];
        *nums.iter().max().unwrap()
    }
}