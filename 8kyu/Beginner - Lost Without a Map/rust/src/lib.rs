fn maps(values: &Vec<i32>) -> Vec<i32> {
    values.iter().map(|x| x * 2).collect()
}

use rand::Rng;

macro_rules! compare {
  ( $got : expr, $expect : expr ) => {
    if $got != $expect { panic!("Got: {:?}\nExpect: {:?}\n", $got, $expect); }
  };
}
fn solve(values: &Vec<i32>) -> Vec<i32> {
    values.iter().map(|x| x*2).collect()
}

#[cfg(test)]
mod tests {
    use self::super::*;

    #[test]
    fn basic_tests() {
        compare!(maps(&vec![1, 2, 3, 4]), vec![2, 4, 6, 8]);
        compare!(maps(&vec![12, 5, 9, 7]), vec![24, 10, 18, 14]);
    }
    #[test]
    fn length_test() {
        compare!(maps(&vec![19037, 2793, 345, 544, 43, 100, 677]), vec![38074, 5586, 690, 1088, 86, 200, 1354]);
    }
    #[test]
    fn negative_test() {
        compare!(maps(&vec![-7, -43, -98, -45, -32, -1123, -1]), vec![-14, -86, -196, -90, -64, -2246, -2]);
    }
    #[test]
    fn mixed_test() {
        compare!(maps(&vec![7, -43, -98, 45, 32, 0, -1, 3]), vec![14, -86, -196, 90, 64, 0, -2, 6]);
    }
    #[test]
    fn empty_test() {
        compare!(maps(&vec![]), Vec::<i32>::new());
    }
    
    #[test]
    fn random_tests() {
        for _ in 0..150 {
            let r_vec = (0..rand::thread_rng().gen_range(0..10))
                .map(|_| rand::thread_rng().gen_range(-5000..5000))
                .collect();
            compare!(maps(&r_vec), solve(&r_vec));
        }
    }
}