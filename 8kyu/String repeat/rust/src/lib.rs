fn repeat_str(src: &str, count: usize) -> String {
    src.repeat(count)
}

// The rust stdlib doesn't include functions for random number generation. 
// This typically provided by the “rand” external library, we unfortunately 
// don’t have access to external libraries. We get around this by linking to
// rand() in the C standard library.
extern {
    fn rand() -> isize;
  }
  
  // Safe wrapper for rand() 
  fn crand() -> isize {
    unsafe { rand() as isize }
  }
  
  fn crand_range(range: std::ops::Range<isize>) -> isize {
    crand() % (range.end - range.start) + range.start
  }
  
  // Working solution
  fn solution(src: &str, count: usize) -> String {
    std::iter::repeat(src).take(count).collect()
  }
  
  // Iterator generating ascci characters
  struct RandAscii;
  impl Iterator for RandAscii {
    type Item = char;
    fn next(&mut self) -> Option<char> {
      let ascii = crand_range(32..126) as u8;
      Some(char::from(ascii))
    }
  }
  
  #[test]
  fn core_tests() {
    assert_eq!(repeat_str("a", 4), "aaaa");
    assert_eq!(repeat_str("hello ", 3), "hello hello hello ");
    assert_eq!(repeat_str("abc", 2), "abcabc");
  }
  
  #[test]
  fn random_tests() {
    for _ in 0..64 {
      let reps = crand_range(1..50) as usize;
      let len = crand_range(1..32) as usize;
      let rstr = RandAscii.take(len).collect::<String>();
      
      assert_eq!(repeat_str(&rstr[..], reps), repeat_str(&rstr[..], reps));
    }
  }