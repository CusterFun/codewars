fn get_char(c: i32) -> char {
    // todo!()
    c as u8 as char
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn fixed() {
        assert_eq!(get_char(55), '7');
        assert_eq!(get_char(56), '8');
        assert_eq!(get_char(57), '9');
        assert_eq!(get_char(58), ':');
        assert_eq!(get_char(59), ';');
        assert_eq!(get_char(60), '<');
        assert_eq!(get_char(61), '=');
        assert_eq!(get_char(62), '>');
        assert_eq!(get_char(63), '?');
        assert_eq!(get_char(64), '@');
        assert_eq!(get_char(65), 'A');
    }

    #[test]
    fn random() {
        fn _get_char(c: i32) -> char {
            c as u8 as char
        }

        let mut rng = rand::thread_rng();
        for _ in 0..200 {
            let n = rng.gen_range(32..127);
            assert_eq!(get_char(n), _get_char(n));
        }
    }
}