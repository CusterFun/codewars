pub fn remove_char(s: &str) -> String {
    let ret = s.chars().collect::<Vec<_>>();
    ret[1..ret.len()-1].into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::remove_char;
    use rand::{Rng, thread_rng};
    use rand::distributions::Alphanumeric;

    #[test]
    fn sample_cases() {
        assert_eq!(remove_char_into("eloquent"), "loquen");
        assert_eq!(remove_char_pop("country"), "ountr");
        assert_eq!(remove_char("person"), "erso");
        assert_eq!(remove_char("place"), "lac");
        assert_eq!(remove_char("ok"), "");
        assert_eq!(remove_char("ooopsss"), "oopss");
    }

    #[test]
    fn random_cases() {
        gen_random_strings(1000, 2, 100).iter()
            .for_each(|string| assert_eq!(remove_char(string), __remove_char(string)));
    }
    
    fn __remove_char(s: &str) -> String {
        let end = s.len() - 1;
        s[1..end].to_string()
    }
    
    fn gen_random_strings(num_strings: usize, min_len: usize, max_len: usize) -> Vec<String> {
        let mut strings: Vec<String> = vec![];
        for _ in 0..num_strings {
            let string = random_string(min_len, max_len);
            strings.push(string);
        }
        strings
    }
    
    fn random_string(min_len: usize, max_len: usize) -> String {
        let mut rng = thread_rng();
        let len = rng.gen_range(min_len..max_len);
        rng.sample_iter(&Alphanumeric)
            .take(len)
            .map(char::from)
            .collect()
    }

    // 标记：into 使用
    fn remove_char_into(s: &str) -> String {
        s[1..s.len()-1].into()
    }

    // 标记：pop 和 remove
    fn remove_char_pop(s: &str) -> String {
        let mut st = s.to_string();
        st.pop();
        st.remove(0);
        st
    }
}