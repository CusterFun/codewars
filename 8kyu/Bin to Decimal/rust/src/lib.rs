fn bin_to_decimal(inp: &str) -> i32 {
    let v: Vec<char> = inp.chars().collect();
    let mut result: i32 = 0;
    for (i, c) in v.iter().rev().enumerate() {
        if *c == '1' {
            let mut temp = 1;
            for _ in 0..i {
                temp *= 2
            }
            result += temp
        }
    }
    result
}


extern crate rand;

use rand::distributions::{Distribution, Uniform};

#[test]
fn test_bin_to_decimal() {
    assert_eq!(bin_to_decimal("0"), 0);
    assert_eq!(bin_to_decimal("1"), 1);
    assert_eq!(bin_to_decimal("10"), 2);
    assert_eq!(bin_to_decimal("11"), 3);
    assert_eq!(bin_to_decimal("101010"), 42);
    assert_eq!(bin_to_decimal("1001001"), 73);
}

#[test]
fn test_bin_to_decimal_random() {
    let step = Uniform::new(1, 5000000);
    let mut random = rand::thread_rng();

    for _index in 0..1000 {
        let sample_value: i32 = step.sample(&mut random);
        let string_value: String = format!("{:b}", sample_value);
        assert_eq!(bin_to_decimal(string_value.as_str()), sample_value);
    }
}

fn bin_to_decimal_best(inp: &str) -> i32 {
    i32::from_str_radix(inp, 2).unwrap_or(0)
}