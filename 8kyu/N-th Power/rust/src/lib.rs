fn index(nums: &[u64], n: usize) -> Option<u64> {
    // todo!();
    if n >= nums.len() {
        return None;
    }
    return Some(nums[n].pow(n as u32));
}

#[cfg(test)]
mod tests {
    use super::index;

    #[test]
    fn fixed_tests() {
        do_test(&[1,2,3,4], 2, Some(9));
        do_test(&[1,3,10,100], 3, Some(1000000));
        do_test(&[1,1], 0, Some(1));
        do_test(&[1,1], 1, Some(1));
        do_test(&[2,2,2], 2, Some(4));
        do_test(&[1,2], 2, None);
        do_test(&[1,2], 3, None);
        do_test(&[6], 0, Some(1));
        do_test(&[1,1,1,1,1,1,1,1,1,1], 9, Some(1));
        do_test(&[1,1,1,1,1,1,1,1,1,100], 9, Some(1_000_000_000_000_000_000));
        do_test(&[29,82,45,10], 3, Some(1000));
        do_test(&[6,31], 3, None);
        do_test(&[75,68,35,61,9,36,89,1,30], 10, None);
    }
    
    fn reference_solution(nums: &[u64], n: usize) -> Option<u64> {
        if n >= nums.len() {
            None
        } else {
            Some(nums[n].pow(n as u32))
        }
    }
    
    #[test]
    fn random_tests() {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        for _ in 0..100 {
            let n = rng.gen_range(0..11);
            let len = rng.gen_range(1..21);
            let nums: Vec<u64> = (0..len).map(|_| rng.gen_range(1..11)).collect();
            let expected = reference_solution(&nums, n);
            do_test(&nums, n, expected);
        }
    }
    
    fn do_test(nums: &[u64], n: usize, exp: Option<u64>) {
        let user_result = index(nums, n);
        assert!(
            user_result == exp,
            "nums: {:?}\nn: {}\ngot:      {:?}\nexpected: {:?}\nTest failed: got is not equal to expected",
            nums,
            n,
            user_result,
            exp
        );
    }

    fn index_best(nums: &[u64], n: usize) -> Option<u64> {
        nums.get(n).map(|x| x.pow(n as u32))
    }

    fn index_match(nums: &[u64], n: usize) -> Option<u64> {
        match nums.get(n) {
            Some(i) => { return Some(i.pow(n as u32)); }
            None => { return None; }
        }
    }
}