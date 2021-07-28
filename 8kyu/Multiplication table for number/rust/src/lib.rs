fn multi_table(n: u64) -> String {
    let mut res = String::from("");
    for i in 1..=10 {
        res += &format!("{} * {} = {}\n", i, n, i * n);
    }
    res.trim_end().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(multi_table(5), "1 * 5 = 5\n2 * 5 = 10\n3 * 5 = 15\n4 * 5 = 20\n5 * 5 = 25\n6 * 5 = 30\n7 * 5 = 35\n8 * 5 = 40\n9 * 5 = 45\n10 * 5 = 50");
        assert_eq!(multi_table(1), "1 * 1 = 1\n2 * 1 = 2\n3 * 1 = 3\n4 * 1 = 4\n5 * 1 = 5\n6 * 1 = 6\n7 * 1 = 7\n8 * 1 = 8\n9 * 1 = 9\n10 * 1 = 10");
    }

    use rand::Rng;

    #[test]
    fn random() {
        let mut rng = rand::thread_rng();

        for _ in 0..100 {
            let n = rng.gen_range(1..=10);

            assert_eq!(multi_table(n), multi_table(n));
            assert_eq!(multi_table(n), multi_table_vec(n));
            assert_eq!(multi_table(n), multi_table_solution(n));
        }
    }

    fn multi_table_solution(n: u64) -> String {
        (1..=10)
            .map(|i| format!("{} * {} = {}", i, n, i * n))
            .collect::<Vec<_>>()
            .join("\n")
    }
    fn multi_table_vec(n: u64) -> String {
        let mut res: Vec<String> = Vec::new();
        for i in 1..11 {
            res.push(format!("{} * {} = {}", i, n, i * n));
        }
        res.join("\n")
    }
}
