fn zero_fuel(distance_to_pump: u32, mpg: u32, gallons: u32) -> bool {
    // unimplemented!();
    distance_to_pump <= mpg * gallons
}


#[cfg(test)]
mod tests {
    use super::zero_fuel;
    use rand;
    use rand::Rng;
    fn sol(distance_to_pump: u32, mpg: u32, gallons: u32) -> bool {
      distance_to_pump <= mpg * gallons
    }
    
    #[test]
    fn sample_tests() {
      assert_eq!(zero_fuel(50, 25, 2), true);
      assert_eq!(zero_fuel(100, 50, 1), false);
    }
    
    #[test]
    fn basic_tests() {
      assert_eq!(zero_fuel(60, 30, 3), true);
      assert_eq!(zero_fuel(70, 25, 1), false);
      assert_eq!(zero_fuel(100, 25, 3), false);
    }
    
    #[test]
    fn random_tests() {
      let mut rnd = rand::thread_rng();
      
      for _ in 0..100 {
        let (d, m, f) = (rnd.gen_range(10u32..100u32), rnd.gen_range(10u32..30u32), rnd.gen_range(1u32..4u32));
        
        assert_eq!(zero_fuel(d, m, f), sol(d, m, f));
      }
    }
}