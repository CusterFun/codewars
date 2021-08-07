fn quarter_of(month: u8) -> u8 {
    if month >= 1 && month <= 3 {
        return 1_u8;
    } else if month > 3 && month <= 6 {
        return 2_u8;
    } else if month > 6 && month <=9 {
        return 3_u8;
    } else {
        return 4_u8;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(quarter_of(3), 1, "Month 3 = quarter 1");
        assert_eq!(quarter_of(8), 3, "Month 8 = quarter 3");
        assert_eq!(quarter_of_best(11), 4, "Month 11 = quarter 4");
    }
    
    use rand::seq::SliceRandom;
    
    #[test]
    fn random() {
        let mut rng = rand::thread_rng();
        
        let mut months = (1..13).collect::<Vec<_>>();
        months.shuffle(&mut rng);
        
        for m in months.into_iter() {
            let expect = quarter_of_solution(m);
            
            assert_eq!(quarter_of(m), expect, "Month {} = quarter {}", m, expect);
        }
    }
    
    // 标记：ceil 用法
    fn quarter_of_solution(m: u8) -> u8 {
        (m as f64 / 3.0).ceil() as u8
    }  


    fn quarter_of_best(month: u8) -> u8 {
        match month {
            1..=3 => 1,
            4..=6 => 2,
            7..=9 => 3,
            10..=12 => 4,
            _ => unreachable!(),
        }
    }

}

