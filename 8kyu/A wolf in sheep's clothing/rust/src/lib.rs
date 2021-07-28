fn warn_the_sheep(queue: &[&str]) -> String {
    let mut res = String::new();
    for i in 0..queue.len() {
        if queue[i] == "wolf" {
            if i == queue.len() - 1 {
                res += &"Pls go away and stop eating my sheep".to_string()
            } else {
                res += &format!("Oi! Sheep number {}! You are about to be eaten by a wolf!", queue.len()-1-i)
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(
            warn_the_sheep(&["sheep", "sheep", "sheep", "sheep", "sheep", "wolf", "sheep", "sheep"]),
            "Oi! Sheep number 2! You are about to be eaten by a wolf!",
        );
        assert_eq!(
            warn_the_sheep(&["sheep", "wolf", "sheep", "sheep", "sheep", "sheep", "sheep"]),
            "Oi! Sheep number 5! You are about to be eaten by a wolf!",
        );
        assert_eq!(
            warn_the_sheep(&["wolf", "sheep", "sheep", "sheep", "sheep", "sheep", "sheep"]),
            "Oi! Sheep number 6! You are about to be eaten by a wolf!",
        );
        assert_eq!(
            warn_the_sheep(&["sheep", "wolf", "sheep"]),
            "Oi! Sheep number 1! You are about to be eaten by a wolf!",
        );
        assert_eq!(
            warn_the_sheep(&["sheep", "sheep", "wolf"]),
            "Pls go away and stop eating my sheep",
        );
    }

    use rand::{thread_rng, Rng};
    
    #[test]
    fn random() {
        let mut rng = thread_rng();
        
        for _ in 0..100 {
            let n = rng.gen_range(1..11);
            let mut queue = vec!["sheep"; n];
            queue[rng.gen_range(0..n)] = "wolf";
            
            assert_eq!(warn_the_sheep(&queue[..]), warn_the_sheep_solution(&queue[..]));
        }
    }
    
    fn warn_the_sheep_solution(queue: &[&str]) -> String {
        if queue.last().unwrap().to_string() == "wolf" {
            return String::from("Pls go away and stop eating my sheep");
        }
        
        format!(
            "Oi! Sheep number {}! You are about to be eaten by a wolf!",
            queue.len() - queue.iter().rposition(|&x| x == "wolf").unwrap() - 1,
        )
    }

    fn warn_the_sheep_best(queue: &[&str]) -> String {
        match queue.iter().rev().position(|&a| a == "wolf").unwrap() {
            0 => format!("Pls go away and stop eating my sheep"),
            n => format!("Oi! Sheep number {}! You are about to be eaten by a wolf!", n)
        }
    }
}

