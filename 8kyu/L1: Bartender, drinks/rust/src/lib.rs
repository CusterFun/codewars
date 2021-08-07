fn get_drink_by_profession(param: &str) -> &'static str {
    // code me!
    match param.to_lowercase().as_str() {
        "jabroni" => "Patron Tequila",
        "school counselor" => "Anything with Alcohol",
        "programmer" => "Hipster Craft Beer",
        "bike gang member" =>	"Moonshine",
        "politician" => "Your tax dollars",
        "rapper" => "Cristal",
        _ => "Beer"
    }
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html
#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn fixed_test_1() {
        assert_eq!(get_drink_by_profession("jabrOni"), "Patron Tequila", "'Jabroni' should map to 'Patron Tequila'");
        assert_eq!(get_drink_by_profession("scHOOl counselor"), "Anything with Alcohol", "'School Counselor' should map to 'Anything with alcohol'");
        assert_eq!(get_drink_by_profession("prOgramMer"), "Hipster Craft Beer", "'Programmer' should map to 'Hipster Craft Beer'");
        assert_eq!(get_drink_by_profession("bike ganG member"), "Moonshine", "'Bike Gang Member' should map to 'Moonshine'");
        assert_eq!(get_drink_by_profession("poLiTiCian"), "Your tax dollars", "'Politician' should map to 'Your tax dollars'");
        assert_eq!(get_drink_by_profession("rapper"), "Cristal", "'Rapper' should map to 'Cristal'");
        assert_eq!(get_drink_by_profession("pundit"), "Beer", "'Pundit' should map to 'Beer'");
        assert_eq!(get_drink_by_profession("Pug"), "Beer", "'Pug' should map to 'Beer'");
    }
    
    #[test]
    fn fixed_test_2() {
        assert_eq!(get_drink_by_profession("jabrOnI"), "Patron Tequila", "'Jabroni' should map to 'Patron Tequila'");
        assert_eq!(get_drink_by_profession("scHOOl COUnselor"), "Anything with Alcohol", "'School Counselor' should map to 'Anything with alcohol'");
        assert_eq!(get_drink_by_profession("prOgramMeR"), "Hipster Craft Beer", "'Programmer' should map to 'Hipster Craft Beer'");
        assert_eq!(get_drink_by_profession("bike GanG member"), "Moonshine", "'Bike Gang Member' should map to 'Moonshine'");
        assert_eq!(get_drink_by_profession("poLiTiCiAN"), "Your tax dollars", "'Politician' should map to 'Your tax dollars'");
        assert_eq!(get_drink_by_profession("RAPPer"), "Cristal", "'Rapper' should map to 'Cristal'");
        assert_eq!(get_drink_by_profession("punDIT"), "Beer", "'Pundit' should map to 'Beer'");
        assert_eq!(get_drink_by_profession("pUg"), "Beer", "'Pug' should map to 'Beer'");
    }
    
    #[test]
    fn random_test_1() {
        let mut rng = rand::thread_rng();
        for _ in 0..40 {
            if rng.gen() {
                let random_profession = get_random_profession();
                assert_eq!(get_drink_by_profession(random_profession.as_str()), expected_result(random_profession.as_str()));
            }
            else {
                let random_string = get_random_string();
                assert_eq!(get_drink_by_profession(random_string.as_str()), expected_result(random_string.as_str()));
            }
        }
    }
    
    fn expected_result(param: &str) -> &'static str {
        match param.to_lowercase().as_str() {
            "jabroni" => "Patron Tequila",
            "school counselor" => "Anything with Alcohol",
            "programmer" => "Hipster Craft Beer",
            "bike gang member" => "Moonshine",
            "politician" => "Your tax dollars",
            "rapper" => "Cristal",
            _ => "Beer"
        }
    }
    
    fn get_random_profession() -> String {
        let professions = vec!["jabroni", "school counselor", "programmer", "bike gang member", "politician", "rapper"];
        let i = rand::thread_rng().gen_range(0..professions.len());
        get_random_case(String::from(professions[i]))
    }
    
    fn get_random_string() -> String {
        let mut rng = rand::thread_rng();
        let mut random_string = String::new();
        for _ in 0..rng.gen_range(6..19) {
            let c = (rng.gen_range(162..188) - 65u8) as char;
            random_string.push(c);
        }
        get_random_case(random_string)
    }
    
    fn get_random_case(start: String) -> String {
        let mut rng = rand::thread_rng();
        start.chars()
             .map(|c| if rng.gen() { 
                 c.to_uppercase().next().unwrap() 
             } else { c })
             .collect::<String>()
    }
}