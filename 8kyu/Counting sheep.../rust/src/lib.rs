fn count_sheep(sheep: &[bool]) -> u8 {
  let mut res :u8 = 0;
  for i in sheep {
    if *i {
      res += 1;
    }
  }
  res
}

#[test]
fn returns_correct_sheep_count() {
  assert_eq!(count_sheep(&[]), 0);
  assert_eq!(count_sheep(&[false]), 0);
  assert_eq!(count_sheep(&[true]), 1);
  assert_eq!(count_sheep(&[true, false]), 1);
  assert_eq!(count_sheep(&[true, true]), 2);
  assert_eq!(count_sheep_best(&[false, false]), 0);
}

fn count_sheep_best(sheep: &[bool]) -> u8 {
    sheep.iter().filter(|&&x| x).count() as u8    
}