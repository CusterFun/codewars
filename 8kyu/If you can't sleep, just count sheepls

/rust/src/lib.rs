// 标记：map和collect生成String
fn count_sheep(n: u32) -> String {
    (1..=n).map(|x| format!("{} sheep...", x)).collect/*::<String>*/()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    fn test_it(n: u32) -> String {
        let mut result = String::new();
        // 标记：push_str用法
        for i in 0..n {
            result.push_str(&format!("{} sheep...", i + 1));
        }
        
        result
    }

    #[test]
    fn returns_expected() {
        assert_eq!(count_sheep(1), "1 sheep...");
        assert_eq!(count_sheep(2), "1 sheep...2 sheep...");
        assert_eq!(count_sheep(3), "1 sheep...2 sheep...3 sheep...");
    }

    #[test]
    fn random_test() {
        for _ in 0..100 {
            let num = rand::thread_rng().gen_range(1..101);
            assert_eq!(count_sheep(num), test_it(num));
        }
    }
}