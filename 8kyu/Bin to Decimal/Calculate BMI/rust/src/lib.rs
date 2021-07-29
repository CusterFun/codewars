fn bmi(weight: u32, height: f32) -> &'static str {
    let bmi = weight as f32 / (height * height);
    if bmi < 18.5 {
        return "Underweight";
    } else if bmi < 25.0 {
        return "Normal";
    } else if bmi < 30.0 {
        return "Overweight";
    }
    return "Obese";
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn basic_tests() {
        assert_eq!(bmi(50, 1.80), "Underweight");
        assert_eq!(bmi(80, 1.80), "Normal");
        assert_eq!(bmi(90, 1.80), "Overweight");
        assert_eq!(bmi(110, 1.80), "Obese");
    }
    
    fn sol(weight: u32, height: f32) -> &'static str {
        let index = weight as f32 / height.powi(2);
        
        match index {
            index if index <= 18.5 => "Underweight",
            index if index <= 25.0 => "Normal",
            index if index <= 30.0 => "Overweight",
            _ => "Obese"
        }
    }
    
    #[test]
    fn random_tests() {
        use rand::*;
        
        let mut rng = rand::thread_rng();
        
        for _ in 1..100 {
            let w = rng.gen_range(30..150) as u32;
            let h = rng.gen_range(110..210) as f32 / 100.0;
            
            assert_eq!(bmi(w, h), sol(w, h));
        }
    }
}