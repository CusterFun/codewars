fn greet(name: &str) -> String {
    format!("Hello, {} how are you doing today?", name)
}

fn solution(n: &str) -> String {
    format!("Hello, {} how are you doing today?", n)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{
        distributions::{Alphanumeric, Uniform},
        prelude::*,
    };

    #[test]
    fn basic_tests() {
        assert_eq!(greet("Ryan"), "Hello, Ryan how are you doing today?");
        assert_eq!(
            greet("Shingles"),
            "Hello, Shingles how are you doing today?"
        );
    }

    #[test]
    fn random_tests() {
        let mut rng = thread_rng();
        let len_distr = Uniform::from(1..=30);

        for _ in 0..40 {
            let len = (&mut rng).sample(len_distr);
            let s = (&mut rng).sample_iter(&Alphanumeric).take(len).map(char::from).collect::<String>();

            assert_eq!(greet(&s), solution(&s));
        }
    }
}