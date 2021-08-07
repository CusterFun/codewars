fn two_sort(arr: &[&str]) -> String {
    arr.iter()
        .min()
        .unwrap()
        .chars()
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .join("***")
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    use rand::seq::SliceRandom;

    fn solution(arr: &[&str]) -> String {
        let mut a = vec![""; arr.len()];
        a[..arr.len()].clone_from_slice(&arr);
        a.sort();
    
        let mut res = String::new();
        for (i, c) in a[0].chars().enumerate() {
            if i == a[0].len()-1 {
                res.push(c);
            } else {
                res.push_str(&String::from(format!("{}***", c)));
            }
        }
    
        res
    }

    #[test]
    fn sample_test_cases() {
        assert_eq!(two_sort(&["bitcoin", "take", "over", "the", "world", "maybe", "who", "knows", "perhaps"]), "b***i***t***c***o***i***n");
        assert_eq!(two_sort(&["turns", "out", "random", "test", "cases", "are", "easier", "than", "writing", "out", "basic", "ones"]), "a***r***e");
    }

    #[test]
    fn more_test_cases() {
        assert_eq!(two_sort(&["lets", "talk", "about", "javascript", "the", "best", "language"]), "a***b***o***u***t");
        assert_eq!(two_sort(&["i", "want", "to", "travel", "the", "world", "writing", "code", "one", "day"]), "c***o***d***e");
        assert_eq!(two_sort(&["Lets", "all", "go", "on", "holiday", "somewhere", "very", "cold"]), "L***e***t***s");
    }
    
    #[test]
    fn random_test_cases() {
        let mut rng = rand::thread_rng();
        let mut str = ["Bitcoin", "LiteCoin", "Ripple", "Dash", "Lisk", "DarkCoin", "Monero", "Ethereum", "Classic", "Mine", "ProofOfWork", "ProofOfStake", "21inc", "Steem", "Dogecoin", "Waves", "Factom", "MadeSafeCoin", "BTC"];
        for _ in 0..100 {
            str.shuffle(&mut rng);
            let start = rand::thread_rng().gen_range(0..str.len() - 5);
            let s = &str[start..start + 4];
            assert_eq!(two_sort(&s), solution(&s));
        }
    }
}