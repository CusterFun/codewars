fn arr(n: usize) -> Vec<u32> {
    // the numbers 0 to n-1
    let mut vec: Vec<u32> = Vec::new();
    for i in 0..n as u32 {
        vec.push(i);
    }
    vec
}

#[cfg(test)]
mod tests {
    use super::arr;

    #[test]
    fn advanced_tests() {
        assert_eq!(arr(0), vec![]);
        assert_eq!(arr_best(10), vec![0,1,2,3,4,5,6,7,8,9]);
    }
    
    #[test]
    fn random_test() {
        use rand::Rng;
        let n = rand::thread_rng().gen_range(5..15);
        // 标注：map用法
        let expected: Vec<u32> = (0..n).map(|x| x as u32).collect();
        assert_eq!(arr(n), expected);
    }

    fn arr_best(n: usize) -> Vec<u32> {
        (0..n as u32).collect()
    }
}