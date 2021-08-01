fn combat(health: f32, damage: f32) -> f32 {
    // unimplemented!();  
    if health - damage > 0.0 {
        return health - damage;
    }
    return 0.0;
}

#[cfg(test)]
extern crate rand;

#[cfg(test)]
mod tests {
    use super::combat;
    use rand;
    use rand::Rng;

    #[test]
    fn example_tests() {
        assert_eq!(combat(100.0, 5.0), 95.0);
        assert_eq!(combat(92.0, 8.0), 84.0);
        assert_eq!(combat(20.0, 30.0), 0.0, "Health cannot go below 0");
    }
    
    #[test]
    fn basic_tests() {
        assert_eq!(combat(50.0, 49.0), 1.0);
        assert_eq!(combat(90.0, 87.0), 3.0);
        assert_eq!(combat(33.0, 33.0), 0.0);
        assert_eq!(combat_best(100.0, 81.0), 19.0);
        assert_eq!(combat(23.0, 27.0), 0.0, "Health cannot go below 0");
    }
    
    #[test]
    fn random_tests() {
        let mut rnd = rand::thread_rng();
        let solution = |h: f32, d: f32| { f32::max(0.0, h - d) }; 
        
        for _ in 0..100 {
            let (h, d) = ((rnd.gen::<f32>() * 400.0).round() / 4.0, 
                          (rnd.gen::<f32>() * 400.0).round() / 4.0);
            
            assert_eq!(combat(h, d), solution(h, d));
        }
    }

    fn combat_best(health: f32, damage: f32) -> f32 {
        (health - damage).max(0.0)
    }
}