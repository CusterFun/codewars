fn greet(language: &str) -> &str {
    match language {
        "english" => "Welcome",
        "czech" => "Vitejte",
        "danish" => "Velkomst",
        "dutch" => "Welkom",
        "estonian" => "Tere tulemast",
        "finnish" => "Tervetuloa",
        "flemish" => "Welgekomen",
        "french" => "Bienvenue",
        "german" => "Willkommen",
        "irish" => "Failte",
        "italian" => "Benvenuto",
        "latvian" => "Gaidits",
        "lithuanian" => "Laukiamas",
        "polish" => "Witamy",
        "spanish" => "Bienvenido",
        "swedish" => "Valkommen",
        "welsh" => "Croeso",
        _ => "Welcome",
    }
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;
    use rand::seq::SliceRandom;
    use lazy_static::lazy_static;
    use std::collections::HashMap;

    #[test]
    fn test_fixed() {
        assert_eq!(greet("english"), "Welcome");
        assert_eq!(greet("dutch"), "Welkom");
        assert_eq!(greet("IP_ADDRESS_INVALID"), "Welcome");
        assert_eq!(greet(""), "Welcome");
        assert_eq!(greet("swelsh"), "Welcome");
    }
    
    #[test]
    fn test_random() {
        // 标注：lazy_static 和 hashMap 使用
        lazy_static! {
            static ref DB_: HashMap<&'static str, &'static str> = [
                ("english", "Welcome"),
                ("czech", "Vitejte"),
                ("danish", "Velkomst"),
                ("dutch", "Welkom"),
                ("estonian", "Tere tulemast"),
                ("finnish", "Tervetuloa"),
                ("flemish", "Welgekomen"),
                ("french", "Bienvenue"),
                ("german", "Willkommen"),
                ("irish", "Failte"),
                ("italian", "Benvenuto"),
                ("latvian", "Gaidits"),
                ("lithuanian", "Laukiamas"),
                ("polish", "Witamy"),
                ("spanish", "Bienvenido"),
                ("swedish", "Valkommen"),
                ("welsh", "Croeso"),
                ("IP_ADDRESS_INVALID", "Welcome"),
                ("IP_ADDRESS_NOT_FOUND", "Welcome"),
                ("IP_ADDRESS_REQUIRED", "Welcome"),
                (" ", "Welcome"),
                ("", "Welcome"),
                ("8314141", "Welcome"),
                ("spanissh", "Welcome"),
                (" vb jhv", "Welcome"),
            ].iter().cloned().collect::<HashMap<_,_>>();
        }
        let mut rng = rand::thread_rng();
        let mut keys = DB_.keys().cloned().collect::<Vec<_>>();
        keys.as_mut_slice().shuffle(&mut rng);
        for key in keys {
            assert_eq!(greet(key.clone()), DB_[key]);
        }
    }
}