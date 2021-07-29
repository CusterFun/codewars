fn digitize(mut n: u64) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();
    while n > 0 {
        res.push((n % 10) as u8);  
        n /= 10
    }
    if res.len() > 0 {
        return res;
    }
    vec![0]
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn test_fixed() {
        assert_eq!(digitize(35231), vec![1,3,2,5,3]);
        assert_eq!(digitize(23582357), vec![7,5,3,2,8,5,3,2]);
        assert_eq!(digitize(984764738), vec![8,3,7,4,6,7,4,8,9]);
        assert_eq!(digitize(45762893920), vec![0,2,9,3,9,8,2,6,7,5,4]);
        assert_eq!(digitize(548702838394), vec![4,9,3,8,3,8,2,0,7,8,4,5]);
    }
    
    #[test]
    fn test_smallest() {
        assert_eq!(digitize(0), vec![0]);
    }
    
    #[test]
    fn test_random() {
        // 标记：数字转字符串，字符串变为字符数组，字符通过 u8 - b'0' 转为数字
        fn solution(n: u64) -> Vec<u8> {
            n.to_string()
             .chars()
             .rev()
             .map(|c| c as u8 - b'0')
             .collect()
        }

        let mut rng = rand::thread_rng();
        for x in 0..37 {
            let y = rng.gen_range(10..99 * 2u64.pow(x));
            let exp = solution(y);
            let sol = digitize(y);
            assert_eq!(sol, exp);
        }

        // 标记：数字转字符再转数字
        fn solution2(n: u64) -> Vec<u8> {
            n
                .to_string()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .rev()
                .collect::<Vec<u8>>()
        }
    }
}