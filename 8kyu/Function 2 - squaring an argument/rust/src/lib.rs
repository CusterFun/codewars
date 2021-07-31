// Define a function "square" here which takes a signed integer of type i32
// and returns the square of that integer
fn square(n : i32) -> i32 {
    n*n
}

#[cfg(test)]
mod tests {
    use super::square;

    #[test]
    fn test_square() {
        assert_eq!(square(1), 1, "expected 1 squared to be 1");
        assert_eq!(square(2), 4, "expected 2 squared to be 4");
        assert_eq!(square(3), 9, "expected 3 squared to be 9");
        assert_eq!(square(4), 16, "expected 4 squared to be 15");
        assert_eq!(square_best(5), 25, "expected 5 squared to be 25");
    }
    
    #[test]
    fn random_tests() {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let n = rng.gen_range(-9999..10000);
            let expected = n * n;
            assert_eq!(square(n), expected, "expected {} squared to be {}", n, expected);
        }
    }

    fn square_best(n: i32) -> i32 {
        n.pow(2)
    }
}