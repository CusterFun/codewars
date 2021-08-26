fn to_time(mut seconds: u32) -> String {
    let mut hours = 0;
    let mut minute = 0;

    loop {
        if seconds >= 3600 {
            hours = seconds / 3600;
            seconds -= hours * 3600;
            println!("hours: {}, seconds: {}", hours, seconds);
        } else if seconds >= 60 {
            minute = seconds / 60;
            seconds -= minute * 60;
            println!("minute: {}, seconds: {}", minute, seconds);
        } else {
            break;
        }
    }

    format!("{} hour(s) and {} minute(s)", hours, minute)
}

#[cfg(test)]
mod tests {
    use super::to_time;

    #[test]
    fn basic() {
        assert_eq!(to_time(3_600), "1 hour(s) and 0 minute(s)");
        assert_eq!(to_time(3_601), "1 hour(s) and 0 minute(s)");
        assert_eq!(to_time(3_500), "0 hour(s) and 58 minute(s)");
        assert_eq!(to_time_best(323_500), "89 hour(s) and 51 minute(s)");
        assert_eq!(to_time(0), "0 hour(s) and 0 minute(s)");
    }
    
    #[test]
    fn random() {
        use rand::{Rng, thread_rng};
        
        let mut rng = thread_rng();
        
        for _ in 0..100 {
            let seconds = rng.gen_range(0..999_999);
            
            let actual = to_time(seconds);
            let expect = to_time_solution(seconds);
            
            assert_eq!(
                actual, expect,
                "for {} seconds",
                seconds,
            );
        }
    }
    
    fn to_time_solution(mut s: u32) -> String {
        let h = s / 3600;
        s -= h * 3600;
        let m = s / 60;
    
        format!("{} hour(s) and {} minute(s)", h, m)
    }

    fn to_time_best(seconds: u32) -> String {
        format!("{} hour(s) and {} minute(s)", seconds / 3600, seconds / 60 % 60)
    }
}