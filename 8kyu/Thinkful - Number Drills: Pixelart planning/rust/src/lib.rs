fn is_divisible(wall: i32, pixel: i32) -> bool {
    wall % pixel == 0
}

// Rust test example:
// TODO: replace with your own tests (TDD), these are just how-to examples.
// See: https://doc.rust-lang.org/book/testing.html

#[test]
fn sample_test() {
  assert_eq!(is_divisible(4050, 27), true);
  assert_eq!(is_divisible(4066, 27), false);
  assert_eq!(is_divisible(2, 4), false);
  assert_eq!(is_divisible(4, 2), true);
}