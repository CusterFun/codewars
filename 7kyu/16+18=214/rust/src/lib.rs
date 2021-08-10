fn add(num1: u32, num2: u32) -> u64 {
    // 1. 数字 反转成 字符串 再转成数组Vec<u8>  比如 112 转成 211，81 转成 18
    let c1:Vec<u8> = num1.to_string().chars().rev().map(|c| c as u8 - b'0').collect();
    let c2:Vec<u8> = num2.to_string().chars().rev().map(|c| c as u8 - b'0').collect();

    println!("c1: {:?}, c2: {:?}", c1, c2); // c1: [2, 2, 1] c2: [1, 8]

    let n1 = c1.len();
    let n2 = c2.len();
    let n = usize::min(n1, n2);
    let mut res = Vec::new();
    // 2. 每一位相加
    for i in 0..n {
        res.push(c1[i] + c2[i]);
    }
    println!("n1: {:?}, n2: {:?}, res1: {:?}", n1, n2, res); // n1: 3, n2: 2, res1: [3, 10]
    // 3. 剩余的数字加入到结果数组
    if n1 > n {
        for i in &c1[n..] {
            res.push(*i);
        }
    }
    println!("res2:{:?}", res); // res2:[3, 10, 1]
    if n2 > n {
        for i in &c2[n..] {
            res.push(*i);
        }
    }
    println!("res3:{:?}", res); // res3:[3, 10, 1]
    // Vec<u8> 数组转成字符串
    let str = res
        .iter()
        .rev()
        .map(|c| format!("{}", c))
        .collect::<String>();
    println!("res4: {:?}", res); // res4: [3, 10, 1]
    println!("str: {:?}", str); // str: "1103"
    str.parse::<u64>().unwrap()
}

extern crate rand;

use self::rand::Rng;

// 标记 zip 用法 format!("{:010}") 数字转字符串
fn solution(num1: u32, num2: u32) -> u64 {
    let (str1, str2) = (format!("{:010}", num1), format!("{:010}", num2));
    str1
        .chars()
        .zip(str2.chars())
        .map(|z| (z.0.to_digit(10).unwrap() as u8 + z.1.to_digit(10).unwrap() as u8).to_string())
        .collect::<String>()
        .parse::<u64>()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_real() {
        assert_eq!(add(2, 11), 13);
        assert_eq!(add(0, 1), 1);
        assert_eq!(add(0, 0), 0);
    }
    
    #[test]
    fn test_silly() {
        assert_eq!(add(16, 18), 214);
        assert_eq!(add(26, 39), 515);
        assert_eq!(add(122, 81), 1103);
    }
    
    #[test]
    fn test_big() {
        assert_eq!(add(1222, 30277), 31499);
        assert_eq!(add(1236, 30977), 31111013);
        assert_eq!(add(38810, 1383), 391193);
    }
    
    #[test]
    fn test_random() {
        for _ in 0..100 {
            let num1 = rand::thread_rng().gen_range(0..4294967295);
            let num2 = rand::thread_rng().gen_range(0..4294967295);
            assert_eq!(add(num1, num2), solution(num1, num2));
        }
    }

    fn add_div(mut x: u32, mut y: u32) -> u64 {
        let mut res = 0;
        let mut m = 1;
        while x != 0 || y != 0 {
            let s = x%10 + y%10;
            res += s as u64 * m;
            m *= if s < 10 { 10 } else { 100 };
            x /= 10;
            y /= 10;
        }
        res
    }
}