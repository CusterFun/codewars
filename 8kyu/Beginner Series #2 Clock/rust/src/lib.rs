fn past(h: i32, m: i32, s: i32) -> i32 {
    let mut res = 1000; // 表示毫秒
    res *= h*60*60+m*60+s;
    res
}

#[cfg(test)]
mod tests {

    use super::*;
    use rand::{thread_rng, Rng};

    #[test]
    fn basic_tests() {
        assert_eq!(past(0, 1, 1), 61000);
        assert_eq!(past(1, 1, 1), 3661000);
        assert_eq!(past(0, 0, 0), 0);
        assert_eq!(past(1, 0, 1), 3601000);
        assert_eq!(past(1, 0, 0), 3600000);
    }
    
    fn sol(h: i32, m: i32, s: i32) -> i32 {
        (h * 3600 + m * 60 + s) * 1000
    }
    
    #[test]
    fn random_tests() {
        let mut rng = thread_rng();
        for _ in 0..100 {
            let h = rng.gen_range(0..23);
            let m = rng.gen_range(0..59);
            let s = rng.gen_range(0..59);
            assert_eq!(past(h, m, s), sol(h, m, s));
        }
    }
}