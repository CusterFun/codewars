fn reverse_words(str:&str) -> String {
    // "backward! is This".to_string()
    str.split_whitespace().rev().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::reverse_words;
    #[test]
    fn returns_expected() {
      let test_cases = [("hello world!"                , "world! hello"),
                        ("yoda doesn't speak like this", "this like speak doesn't yoda"),
                        ("foobar"                      , "foobar"),
                        ("kata editor"                 , "editor kata"),
                        ("row row row your boat"       , "boat your row row row")];
      for &(string, reverse) in test_cases.into_iter() {
        assert_eq!(reverse_words(string), reverse);
      }
    }
}