fn find_smallest_int(arr: &[i32]) -> i32 {
    // your code here
    let mut min = arr[0];
    for i in arr {
        if *i < min {
            min = *i;
        }
    }
    min
}

extern crate rand;
#[allow(unused_imports)]
use self::rand::Rng;

fn solution(arr: &[i32]) -> i32 {
    *arr.iter().min().unwrap()
}

#[test]
fn sample_tests() {
    assert_eq!(find_smallest_int(&[34, 15, 88, 2]), 2);
    assert_eq!(find_smallest_int(&[34, -345, -1, 100]), -345);
}

#[test]
fn more_tests() {
  assert_eq!(find_smallest_int(&[78,56,232,12,8]), 8);
  assert_eq!(find_smallest_int(&[78,56,232,12,18]),12);
  assert_eq!(find_smallest_int(&[78,56,232,412,228]),56);
  assert_eq!(find_smallest_int(&[78,56,232,12,0]),0);
  assert_eq!(find_smallest_int(&[-1,56,232,12,8]),-1);
}

#[test]
fn random_tests() {
  for _ in 0..10 {
        let len = rand::thread_rng().gen_range(5..50);
        let mut vec = Vec::new();
        for _ in 0..len {
            vec.push(rand::thread_rng().gen_range(-100..100));
        }
        
        assert_eq!(find_smallest_int(&vec[..len]), solution(&vec[..len]));
    }
}