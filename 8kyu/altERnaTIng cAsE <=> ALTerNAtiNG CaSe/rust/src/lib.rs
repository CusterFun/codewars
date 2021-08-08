// 标注：is_uppercase 和 to_uppercase 使用
fn to_alternating_case(s: &str) -> String {
    s.chars().map(|c| {
        match c {
            c if c.is_uppercase() => c.to_lowercase().to_string(),
            c if c.is_lowercase() => c.to_uppercase().to_string(),
            _ => c.to_string(),
        }
    }).collect()
}

#[cfg(test)]
mod tests {
  use super::*;
  use rand::{Rng, thread_rng};
  use rand::distributions::Alphanumeric;

  // Test solution without unicode support
  fn test_solution(s: &str) -> String {
    s.chars().map(|c| 
      if c.is_lowercase() {
        c.to_ascii_uppercase()
      } else if c.is_uppercase() {
        c.to_ascii_lowercase()
      } else {
        c
      }
    ).collect()
  }

  #[test]
  fn example_tests() {
    assert_eq!("HELLO WORLD", to_alternating_case("hello world"));
    assert_eq!("hello world", to_alternating_case("HELLO WORLD"));
    assert_eq!("HELLO world", to_alternating_case("hello WORLD"));
    assert_eq!("hEllO wOrld", to_alternating_case("HeLLo WoRLD"));
    assert_eq!("Hello World", to_alternating_case(&to_alternating_case("Hello World")[..]));
    assert_eq!("12345", to_alternating_case("12345"));
    assert_eq!("1A2B3C4D5E", to_alternating_case("1a2b3c4d5e"));
    assert_eq!("sTRING.tOaLTERNATINGcASE", to_alternating_case("String.ToAlternatingCase"));
  }

  #[test]
  fn random_tests() {
    for _ in 0..50 {
      let mut rng = thread_rng();
      let l = rng.gen_range(5..50);
      let s: String = rng.sample_iter(&Alphanumeric).take(l).map(char::from).collect();
      assert_eq!(test_solution(&s[..]), to_alternating_case(&s[..]));
    }
  }
}