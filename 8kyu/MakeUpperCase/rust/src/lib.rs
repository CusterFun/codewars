fn make_upper_case(s: &str) -> String {
    // code here
    s.to_uppercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_upper_case() {
        assert_eq!(make_upper_case("hello"), "HELLO");
        assert_eq!(make_upper_case("hello world"), "HELLO WORLD");
        assert_eq!(make_upper_case("hello world !"), "HELLO WORLD !");
        assert_eq!(make_upper_case("heLlO wORld !"), "HELLO WORLD !");
        assert_eq!(make_upper_case("1,2,3 hello world"), "1,2,3 HELLO WORLD");
    }
}