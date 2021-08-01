fn hero(bullets: u16, dragons: u16) -> bool {
    bullets >= 2*dragons
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(hero(10, 5), true);
        assert_eq!(hero(7, 4), false);
        assert_eq!(hero(4, 5), false);
        assert_eq!(hero(100, 40), true);
        assert_eq!(hero(1500, 751), false);
        assert_eq!(hero(0, 1), false);
    }
    
    use rand::Rng;
    
    #[test]
    fn random() {
        let mut rng = rand::thread_rng();
        
        for _ in 0..100 {
            let bullets = rng.gen_range(0..=1_000);
            let dragons = rng.gen_range(0..bullets);
            
            assert_eq!(hero(bullets, dragons), hero_solution(bullets, dragons));
        }
    }
    
    fn hero_solution(bullets: u16, dragons: u16) -> bool {
        bullets >= dragons * 2
    }
}