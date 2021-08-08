fn subtract_sum(n: u32) -> &'static str {
    let list = vec!["",
        "kiwi", "pear", "kiwi", "banana", "melon", "banana", "melon", "pineapple", "apple", "pineapple",
        "cucumber", "pineapple", "cucumber", "orange", "grape", "orange", "grape", "apple", "grape", "cherry",
        "pear", "cherry", "pear", "kiwi", "banana", "kiwi", "apple", "melon", "banana", "melon",
        "pineapple", "melon", "pineapple", "cucumber", "orange", "apple", "orange", "grape", "orange", "grape",
        "cherry", "pear", "cherry", "pear", "apple", "pear", "kiwi", "banana", "kiwi", "banana",
        "melon", "pineapple", "melon", "apple", "cucumber", "pineapple", "cucumber", "orange", "cucumber", "orange",
        "grape", "cherry", "apple", "cherry", "pear", "cherry", "pear", "kiwi", "pear", "kiwi",
        "banana", "apple", "banana", "melon", "pineapple", "melon", "pineapple", "cucumber", "pineapple", "cucumber",
        "apple", "grape", "orange", "grape", "cherry", "grape", "cherry", "pear", "cherry", "apple",
        "kiwi", "banana", "kiwi", "banana", "melon", "banana", "melon", "pineapple", "apple", "pineapple"
    ];
    let mut num = n;
    num -= sum(num);
    while num > 101  {
        num -=  sum(num);
    }
    list[num as usize]
}

fn sum(mut num: u32) -> u32 {
    let mut sum = 0;

    while num > 0 {
        sum += num % 10;
        num /= 10;
    }
    sum
}

#[cfg(test)]
mod tests {    
    use super::*;
    use rand::{thread_rng, Rng};

    #[test]
    fn basic_tests() {
        assert_eq!(subtract_sum(10), "apple");
    }
    
    #[test]
    fn random_tests() {
        let mut rng = thread_rng();
        
        for _ in 0..100 {
            assert_eq!(subtract_sum(rng.gen_range(10..10000)), "apple");
        }
    }
}