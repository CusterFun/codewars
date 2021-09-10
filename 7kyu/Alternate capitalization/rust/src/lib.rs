fn capitalize(s: &str) -> Vec<String> {
    let mut even =Vec::new();
    let mut odd =Vec::new();
    for (i, c) in s.chars().enumerate() {
        if i == 0 {
            even.push(c.to_ascii_uppercase());
            odd.push(c);
        } else if i % 2== 0 {
            even.push(c.to_ascii_uppercase());
            odd.push(c);
        } else {
            even.push(c);
            odd.push(c.to_ascii_uppercase());
        }
    }
    vec![even.into_iter().collect(), odd.into_iter().collect()]
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html
#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn basic_tests() {
        assert_eq!(capitalize("abcdef"),["AbCdEf", "aBcDeF"]);
        assert_eq!(capitalize("codewars"),["CoDeWaRs", "cOdEwArS"]);
        assert_eq!(capitalize("abracadabra"),["AbRaCaDaBrA", "aBrAcAdAbRa"]);
        assert_eq!(capitalize("codewarriors"),["CoDeWaRrIoRs", "cOdEwArRiOrS"]);
        assert_eq!(capitalize("indexinglessons"),["InDeXiNgLeSsOnS", "iNdExInGlEsSoNs"]);
        assert_eq!(capitalize("codingisafunactivity"),["CoDiNgIsAfUnAcTiViTy", "cOdInGiSaFuNaCtIvItY"]);
    }
    
    #[test]
    fn random_tests() {
        for _ in 0..100 {
            let random_string = get_random_string();
            assert_eq!(capitalize(random_string.as_str()), expected_result(random_string.as_str()));
        }
    }
    
    fn expected_result(s: &str) -> Vec<String> {
        let even = s.chars().enumerate().map(|(i, c)| if i % 2 == 0 { c.to_uppercase().next().unwrap() } else { c }).collect();
        let odd = s.chars().enumerate().map(|(i, c)| if i % 2 == 1 { c.to_uppercase().next().unwrap() } else { c }).collect();
        vec![even, odd]
    }
    
    fn get_random_string() -> String {
        let mut rng = rand::thread_rng();
        let mut random_string = String::new();
        for _ in 0..rng.gen_range(10..20) {
            let c = (rng.gen_range(162..188) - 65u8) as char;
            random_string.push(c);
        }
        random_string
    }
}