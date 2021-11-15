/// 当且仅当一个数的平方结尾与该数本身的数字相同时，该数被称为自动数

fn automorphic(n: u64) -> String {
    let num_str = n.pow(2).to_string();
    let n_str = n.to_string();
    if num_str.ends_with(&n_str) {
        return "Automorphic".to_string();
    }
    "Not!!".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(automorphic(1), "Automorphic");
        assert_eq!(automorphic(3), "Not!!");
        assert_eq!(automorphic(6), "Automorphic");
        assert_eq!(automorphic(9), "Not!!");
        assert_eq!(automorphic(25), "Automorphic");
        assert_eq!(automorphic(53), "Not!!");
        assert_eq!(automorphic(76), "Automorphic");
        assert_eq!(automorphic(95), "Not!!");
        assert_eq!(automorphic_str(625), "Automorphic");
        assert_eq!(automorphic_best(225), "Not!!");
    }

    fn automorphic_str(n: u64) -> String {
        if n.pow(2).to_string().ends_with(&n.to_string()) {
            String::from("Automorphic")
        } else {
            String::from("Not!!")
        }
    }

    fn automorphic_best(mut n: u64) -> String {
        let mut q = n.pow(2);
        while n != 0 {
            if n % 10 != q % 10 {
                return String::from("Not!!");
            }
            n /= 10;
            q /= 10;
        }
        "Automorphic".to_string()
    }

    use rand::{seq::SliceRandom, Rng};

    // All automorphic numbers of which the square is less than u64
    const AUTOMORPHIC: [u64; 19] = [
        0, 1, 5, 6, 25, 76, 376, 625, 9376, 90625, 109376, 890625, 2890625, 7109376, 12890625,
        87109376, 212890625, 787109376, 1787109376,
    ];

    #[test]
    fn random() {
        let mut rng = rand::thread_rng();

        for _ in 0..100 {
            let n = if rng.gen_ratio(1, 2) {
                *AUTOMORPHIC.choose(&mut rng).unwrap()
            } else {
                let rand_exp = rng.gen_range(1..32);

                // Random integer [1, 2_147_483_648] as 2_147_483_648
                // is <= the square root of u64::MAX
                rng.gen_range(1..2u64.pow(rand_exp))
            };

            assert_eq!(automorphic(n), automorphic_solution(n));
        }
    }

    fn automorphic_solution(n: u64) -> String {
        if n.pow(2).to_string().ends_with(&n.to_string()) {
            String::from("Automorphic")
        } else {
            String::from("Not!!")
        }
    }
}
