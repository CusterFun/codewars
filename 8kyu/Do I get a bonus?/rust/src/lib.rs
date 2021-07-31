fn bonus_time(salary: u64, bonus: bool) -> String {
    match bonus {
        true => return format!("¥{}", salary*10),
        false => return format!("¥{}", salary)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{thread_rng, Rng};

    #[test]
    fn test_basic_cases() {
        assert_eq!(bonus_time(10000, true), "¥100000");
        assert_eq!(bonus_time(25000, true), "¥250000");
        assert_eq!(bonus_time(10000, false), "¥10000");
        assert_eq!(bonus_time(60000, false), "¥60000");
        assert_eq!(bonus_time(2, true), "¥20");
        assert_eq!(bonus_time_best(78, false), "¥78");
        assert_eq!(bonus_time_best(67890, true), "¥678900");
    }
    
    #[test]
    fn test_no_bonus() {
        let mut rng = thread_rng();
        for _ in 1..30 {
            let salary = rng.gen_range(0..10000);
            let answer = format!("¥{}", salary);
            assert_eq!(bonus_time(salary, false), answer);        
        }
    }
    
    #[test]
    fn test_with_bonus() {
        let mut rng = thread_rng();
        for _ in 1..30 {
            let salary = rng.gen_range(0..10000);
            let answer = format!("¥{}0", salary);
            assert_eq!(bonus_time(salary, true), answer);        
        }
    }

    // 标记：true as u64
    fn bonus_time_best(salary: u64, bonus: bool) -> String {
        format!("¥{}", salary * (1 + 9 * bonus as u64))
    }
}