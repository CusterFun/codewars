fn boolean_to_string(b: bool) -> String {
    if b {
        return String::from("true");
    }
    return String::from("false");
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn basic() {
        assert_eq!(boolean_to_string(true), "true", "When we pass in true, we want the string \"true\" as output");
        assert_eq!(boolean_to_string(false), "false", "When we pass in false, we want the string \"false\" as output");
        assert_eq!(boolean_to_string(false), "false", "When we pass in false, we want the string \"false\" as output");
        assert_eq!(boolean_to_string(true), "true", "When we pass in true, we want the string \"true\" as output");
        assert_eq!(boolean_to_string(true), "true", "When we pass in true, we want the string \"true\" as output");
        assert_eq!(boolean_to_string(false), "false", "When we pass in false, we want the string \"false\" as output");
    }
    
    #[test]
    fn random() {
        fn _boolean_to_string(b: bool) -> String {
            b.to_string()
        }

        let mut rng = rand::thread_rng();
        for _ in 0..50 {
            let b = rng.gen();
            assert_eq!(boolean_to_string(b), _boolean_to_string(b));
        }
    }

    fn boolean_to_string_best(b: bool) -> String {
        b.to_string()
    }

    fn boolean_to_string_good(b: bool) -> String {
        match b {
            true => String::from("true"),
            false => String::from("false"),
        }
    }
}

