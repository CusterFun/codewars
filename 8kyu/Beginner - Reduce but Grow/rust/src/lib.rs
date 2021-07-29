fn grow(nums: Vec<i32>) -> i32 {
    let mut accum = 1;
    for num in nums.iter() {
        accum *= num
    }
    return accum;
}

#[test]
fn basic_test() {
  assert_eq!(grow(vec![1, 2, 3]), 6);
  assert_eq!(grow_best(vec![4, 1, 1, 1, 4]), 16);
  assert_eq!(grow_fold(vec![2, 2, 2, 2, 2, 2]), 64);
}

fn grow_best(array: Vec<i32>) -> i32 {
    array.iter().product()
}

fn grow_fold(array: Vec<i32>) -> i32 {
    array.iter().fold(1, |acc ,x| acc * x)
}
