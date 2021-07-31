fn format_money(amount: f64) -> String {
    // todo!()
    format!("${value:.*}", 2, value = amount)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(format_money(39.99), "$39.99");
        assert_eq!(format_money(3.0), "$3.00");
        assert_eq!(format_money(3.10), "$3.10");
        assert_eq!(format_money(314.16), "$314.16");
    }
    
    use rand::Rng;
    
    #[test]
    fn random() {
        let mut rng = rand::thread_rng();
        
        // Test numbers without a digit after decimal point 
        for _ in 0..50 {
            let n = (rng.gen::<f64>() * rng.gen_range(1.0..10.0) * 100.0).round() as f64;
            assert_eq!(format_money(n), format_money_solution(n));
        }
        
        // Test numbers with one digit after decimal point
        for _ in 0..50 {
            let n = ((rng.gen::<f64>() * rng.gen_range(1.0..10.0) * 1_000.0).round() as f64) / 10.0;
            assert_eq!(format_money(n), format_money_solution(n));
        }
        
        // Test numbers with two digits after decimal point
        for _ in 0..50 {
            let n = ((rng.gen::<f64>() * rng.gen_range(1.0..10.0) * 10_000.0).round() as f64) / 100.0;
            assert_eq!(format_money(n), format_money_solution(n));
        }
    }
    
    fn format_money_solution(n: f64) -> String {
        format!("${:.2}", n)
    }    
}