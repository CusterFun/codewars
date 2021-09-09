fn solve(strings: &[String]) -> Vec<usize> {
    let mut res:Vec<usize> = Vec::new();
    for str in strings {
        let mut count:usize = 0;
        for (index, c) in (*str).chars().enumerate() {
            if c >= 'a' && c <= 'z' && c as u8 - b'a' == index as u8 || c >= 'A' && c <= 'Z' && c as u8 - b'A' == index as u8  {
                count += 1;
            }
        }
        res.push(count);
    }
    res
}

// https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html
#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn basic_tests() {
        assert_eq!(
            solve(&["abode".to_string(), "ABc".to_string(), "xyzD".to_string()]),
            vec![4, 3, 1]
        );
        assert_eq!(
            solve(&["abide".to_string(), "ABc".to_string(), "xyz".to_string()]),
            vec![4, 3, 0]
        );
        assert_eq!(
            solve(&[
                "IAMDEFANDJKL".to_string(),
                "thedefgh".to_string(),
                "xyzDEFghijabc".to_string()
            ]),
            vec![6, 5, 7]
        );
        assert_eq!(
            best_solve(&[
                "encode".to_string(),
                "abc".to_string(),
                "xyzD".to_string(),
                "ABmD".to_string()
            ]),
            vec![1, 3, 1, 3]
        );
    }

    fn reference_solution(strs: &[String]) -> Vec<usize> {
        let is_alphabet_symmetric = |tuple: &(usize, u8)| tuple.0 == (tuple.1 - b'a') as usize;
        let symmetry_counter = |s: &String| {
            s.to_ascii_lowercase()
                .bytes()
                .enumerate()
                .filter(is_alphabet_symmetric)
                .count()
        };
        strs.iter().map(symmetry_counter).collect()
    }

    #[test]
    fn random_tests() {
        for _ in 0..100 {
            let input = generate_random_input();
            let expected = reference_solution(&input);
            assert_eq!(solve(&input), expected);
        }
    }

    fn generate_random_input() -> Vec<String> {
        use rand::Rng;
        const LOWER: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
        const UPPER: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let mut rng = rand::thread_rng();

        let len = rng.gen_range(98..199);

        (0..len)
            .map(|_| {
                let r0 = rng.gen_range(0..10);
                let r1 = rng.gen_range(0..10);
                let r2 = rng.gen_range(0..26);
                
                let mut s: String = UPPER
                    .iter()
                    .take(r1)
                    .map(|b| *b as char)
                    .collect::<String>();
                
                s += &(LOWER
                    .iter()
                    .take(r0)
                    .map(|b| *b as char)
                    .collect::<String>());
                
                if r0 + r1 < r2 {
                    s += &(LOWER
                        .iter()
                        .skip(r0 + r1)
                        .take(r2 - r1 - r0)
                        .map(|b| *b as char)
                        .collect::<String>());
                }
                
                s
            })
            .collect()
    }

    fn best_solve(strings: &[String]) -> Vec<usize> {
        strings.iter()
            .map(|s| s.to_ascii_lowercase()
                .chars()
                .zip('a'..='z')
                .filter(|(c0,c1)| c0 == c1)
                .count())
            .collect()
    }
}
