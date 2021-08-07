fn get_age(age: &str) -> u32 {
    // Your code here
    age.split(' ').collect::<Vec<&str>>()[0].parse::<u32>().unwrap()
}

#[test]
fn basic_tests() {
  assert_eq!(get_age("1 year old"), 1);
  assert_eq!(get_age("2 years old"), 2);
  assert_eq!(get_age("3 years old"), 3);
  assert_eq!(get_age("4 years old"), 4);
  assert_eq!(get_age("5 years old"), 5);
  assert_eq!(get_age_parse("6 years old"), 6);
  assert_eq!(get_age("7 years old"), 7);
  assert_eq!(get_age("8 years old"), 8);
  assert_eq!(get_age("9 years old"), 9);
}

fn get_age_parse(age: &str) -> u32 {
    age[..1].parse().unwrap()
}