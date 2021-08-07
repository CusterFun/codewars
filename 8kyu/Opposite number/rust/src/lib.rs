fn opposite(number: i32) -> i32 {
    number * -1
}

#[test]
fn test_positive() {
  for i in 0..100 {
    assert_eq!(opposite_best(i), -i);
  }
}

#[test]
fn test_negative() {
  for i in -100..0 {
    assert_eq!(opposite(i), -i);
  }
}


fn opposite_best(number: i32) -> i32 {
    -number
}