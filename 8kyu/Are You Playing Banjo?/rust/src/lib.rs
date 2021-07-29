fn are_you_playing_banjo(name: &str) -> String {
    let names: Vec<char> = name.chars().collect();
    if names[0] == 'R' || names[0] == 'r' {
        return format!("{} plays banjo", name);
    }
    format!("{} does not play banjo", name)
}

// Add your tests here.
// https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::are_you_playing_banjo;

    #[test]
    fn test_extended_are_you_playing_banjo() {
        assert_eq!(are_you_playing_banjo("Adam"), "Adam does not play banjo");
        assert_eq!(are_you_playing_banjo("Paul"), "Paul does not play banjo");
        assert_eq!(are_you_playing_banjo("Ringo"), "Ringo plays banjo");
        assert_eq!(are_you_playing_banjo("bravo"), "bravo does not play banjo");
        assert_eq!(are_you_playing_banjo("rolf"), "rolf plays banjo");
        assert_eq!(are_you_playing_banjo("Martin"), "Martin does not play banjo");
        assert_eq!(are_you_playing_banjo("Rikke"), "Rikke plays banjo");
    }

    #[test]
    fn random_tests() {
        for _ in 0..50 {
            let name = create_random_name();
            let expected = reference_banjo(&name);
            println!("Testing name \"{}\"", name);
            assert_eq!(are_you_playing_banjo(&name), expected);
        }
    }

    fn create_random_string(size: usize) -> String {
        use rand::Rng;
        const LETTERS: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
        let mut rng = rand::thread_rng();

        (0..size)
            .map( |_|
                LETTERS[rng.gen_range(0..LETTERS.len())] as char)
            .collect()
    }

    fn capitalize_first(s: &str) -> String {
        s.chars()
            .enumerate()
            .map(|(i, c)| if i == 0 { c.to_ascii_uppercase() } else { c })
            .collect()
    }
    
    fn force_r_as_first_char(s: &str) -> String {
        s.chars()
            .enumerate()
            .map(|(i, c)| if i == 0 { 'r' } else {c})
            .collect()
    }

    fn create_random_name() -> String {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        let capitalize: bool = rng.gen();
        let forced_r: bool = rng.gen();
        let name_len = rng.gen_range(1..16);

        let mut name = create_random_string(name_len);
        if forced_r {
            name = force_r_as_first_char(&name);
        }
        if capitalize {
            name = capitalize_first(&name);
        }

        name
    }

    fn reference_banjo(name: &str) -> String {
    name.to_owned() +
        match name.chars().nth(0).unwrap() {
            'r' => " plays banjo",
            'R' => " plays banjo",
            _ => " does not play banjo",
        }
    }

    fn are_you_playing_banjo_best(name: &str) -> String {
        match &name[..1] {
            "R" | "r" => format!("{} plays banjo", name),
            _ => format!("{} does not play banjo", name)
        }
    }
}