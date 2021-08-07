fn square_sum(vec: Vec<i32>) -> i32 {
    // let mut res = 0;
    // for i in vec {
    //     res += i.pow(2)
    // }
    // res
    vec.iter().fold(0, |sum, x| sum + x.pow(2))
}

#[test]
fn returns_expected() {
  assert_eq!(square_sum(vec![1, 2]), 5);
  assert_eq!(square_sum(vec![5, 3, 4]), 50);
  assert_eq!(square_sum(vec![-1, -2]), 5);
  assert_eq!(square_sum(vec![-1, 0, 1]), 2);
  assert_eq!(square_sum(vec![-1, 1]), 2);
  assert_eq!(square_map_sum(vec![]), 0);
}

// 标记：map、sum使用
fn square_map_sum(vec: Vec<i32>) -> i32 {
    vec.iter().map(|s| s * s).sum()
}