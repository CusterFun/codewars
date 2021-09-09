fn add_letters(letters: Vec<char>) -> char {
    let mut sum: u8 = 0;
    for c in letters {
        sum += c as u8 - b'a' + 1;
    }
    if sum == 0 {
        return 'z';
    }
    ((sum -1) % 26 + b'a') as char
}

#[cfg(test)] extern crate rand;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(add_letters(vec!['a', 'b', 'c']), 'f');
        assert_eq!(add_letters(vec!['z']), 'z');
        assert_eq!(add_letters(vec!['a', 'b']), 'c');
        assert_eq!(add_letters(vec!['c']), 'c');
        assert_eq!(add_letters(vec!['z', 'a']), 'a');
        assert_eq!(best_add_letters(vec!['y', 'c', 'b']), 'd');
        assert_eq!(add_letters(vec![]), 'z');
    }
    
    fn sol(letters: Vec<char>) -> char {
        "zabcdefghijklmnopqrstuvwxy".chars().nth((letters.iter().map(|x| (*x as i32) - 96).sum::<i32>() % 26) as usize).unwrap()
    }
    
    #[test]
    fn random_tests() {
        use tests::rand::*;
        
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let letters: Vec<char> = (0..=rng.gen_range(0..10)).map(|_| rng.gen_range(97..122) as u8 as char).collect();
            assert_eq!(add_letters(letters.to_owned()), sol(letters.to_owned()));
        }
    }

    fn best_add_letters(letters: Vec<char>) -> char {
        let out = letters.iter().fold(0, |acc, &x| acc + x as i32 - 96) % 26;
        if out == 0 {
            return 'z';
        }
        (out + 96) as u8 as char
    }
}