fn find_multiples(n: u32, limit: u32) -> Vec<u32> {
    // todo!()
    let mut res:Vec<u32> = Vec::new();
    let mut i = 1;
    loop {
        if n * i > limit { break; }
        res.push(n * i);
        i += 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::find_multiples;

    #[test]
    fn basic_test() {
        assert_eq!(find_multiples(1, 2), [1, 2]);
        assert_eq!(find_multiples(5, 7), [5]);
        assert_eq!(find_multiples(4, 27), [4, 8, 12, 16, 20, 24]);
        assert_eq!(find_multiples_clever(11, 54), [11, 22, 33, 44]);
        assert_eq!(find_multiples_best(5, 25), [5, 10, 15, 20, 25]);
    }

    #[test]
    fn random_test() {
        use rand::{Rng, thread_rng};

        let mut rng = thread_rng();

        for _ in 0..100 {
            let rand_n = rng.gen_range(1..13);
            let rand_limit = rng.gen_range(rand_n..rand_n * 100);
            
            assert_eq!(
                find_multiples(rand_n, rand_limit),
                find_multiples_solution(rand_n, rand_limit),
            );
        }
    }
    
    // 标记：step_by 用法
    fn find_multiples_solution(n: u32, limit: u32) -> Vec<u32> {
        (n..=limit)
            .step_by(n as usize)
            .collect()
    }    

    // 标记：take_while 用法
    fn find_multiples_best(n: u32, limit: u32) -> Vec<u32> {
        // (1..=limit/n).map(|x| x * n).collect()
        (1..).map(|x| x * n).take_while(|x| x <= &limit).collect()
    }

    // 标记：filter 用法
    fn find_multiples_clever(n: u32, limit: u32) -> Vec<u32> {
        (n..=limit).filter(|v| v % n == 0).collect()
    }
}