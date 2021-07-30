fn bool_to_word(value: bool) -> &'static str {
    match value {
        true => "Yes",
        false => "No",
    }
}

// https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html
#[cfg(test)]
mod tests {
    use super::bool_to_word;

    #[test]
    fn example_tests() {
        assert_eq!(bool_to_word(true), "Yes");
        assert_eq!(bool_to_word(false), "No");
    }
    
    #[test]
    fn random_tests() {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let value = rng.gen::<bool>();
            let expected = match value {
                true => "Yes",
                false => "No",
            };
            assert_eq!(bool_to_word(value), expected, "\nFailed with value {:?}", value);
        }
    }
}

