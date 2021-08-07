// 标注：contains 使用
fn am_i_wilson(n: u32) -> bool {
    [5, 13, 563].contains(&n)
}


use rand::Rng;
use rand::seq::SliceRandom;

#[test]
fn should_return_false_for_composite_numbers() {
    assert_eq!(am_i_wilson(9), false);
    assert_eq!(am_i_wilson(6), false);
    assert_eq!(am_i_wilson(221), false);
}

#[test]
fn should_return_false_for_zero_and_one() {
    assert_eq!(am_i_wilson(0), false);
    assert_eq!(am_i_wilson(1), false);
}

#[test]
fn should_return_false_for_nonwilson_primes() {
    assert_eq!(am_i_wilson(2), false);
    assert_eq!(am_i_wilson(17), false);
    assert_eq!(am_i_wilson(71), false);
    assert_eq!(am_i_wilson(4801), false);
}

#[test]
fn should_return_true_for_wilson_primes() {
    assert_eq!(am_i_wilson(5), true);
    assert_eq!(am_i_wilson(13), true);
    assert_eq!(am_i_wilson(563), true);
}

#[test]
fn should_return_false_for_wilson_composites() {
    assert_eq!(am_i_wilson(5971), false);
}

#[test]
fn should_pass_random_tests() {
    let mut rng = rand::thread_rng();
    let mut ns: Vec<u32> = (0..50).map(|_| rng.gen_range(0..1500)).collect();
    ns.extend(&[5, 13, 563]);
    ns.shuffle(&mut rng);
    for n in ns {
        let expected = n == 5 || n == 13 || n == 563;
        assert_eq!(am_i_wilson(n), expected);
    }
}