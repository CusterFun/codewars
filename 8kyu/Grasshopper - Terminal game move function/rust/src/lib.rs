fn move_hero(position: u32, roll: u32) -> u32 {
    // unimplemented!();
    position + roll * 2
}

#[cfg(test)]
mod tests {
    use super::move_hero;
    use rand;
    use rand::Rng;

    #[test]
    fn basic_tests() {
        assert_eq!(move_hero(0, 4), 8);
        assert_eq!(move_hero(3, 6), 15);
        assert_eq!(move_hero(2, 5), 12);
    }
    
    #[test]
    fn random_tests() {
        for _ in 0..1000 {
            let (position, roll) = 
                (rand::thread_rng().gen_range(0..101), rand::thread_rng().gen_range(0..101));
                
            assert_eq!(move_hero(position, roll),
                position + roll * 2);
        }
    }
}