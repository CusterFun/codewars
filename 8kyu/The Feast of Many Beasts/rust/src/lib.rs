fn feast(beast: &str, dish: &str) -> bool {
    // your code here
    if beast.chars().nth(0) == dish.chars().nth(0) && 
        beast.chars().nth(beast.len() - 1) == dish.chars().nth(dish.len() - 1) {
        return true;
    } else {
        return false;
    }
}

// 标记：next和last使用
fn feast_next(beast: &str, dish: &str) -> bool {
    beast.chars().next() == dish.chars().next()
    && beast.chars().last() == dish.chars().last()
}
// Rust test cases:
extern crate rand;
#[allow(unused_imports)]
use self::rand::Rng;

fn make_string(length:u8) -> String {
    let mut result = String::new();
    let letters = String::from("abcdefghijklmnopqrstuvwxyz");
    
    for _ in 0..length as usize {
        let x = rand::thread_rng().gen_range(0..letters.len() - 1);
        result.push_str(&letters[x..x+1]);
    }
    result
}

#[test]
fn returns_expected() {
  assert_eq!(feast_next("great blue heron", "garlic naan"), true);
  assert_eq!(feast("chickadee", "chocolate cake"), true);
  assert_eq!(feast("brown bear", "bear claw"), false);
  assert_eq!(feast("marmot", "mulberry tart"), true);
  assert_eq!(feast("porcupine", "pie"), true);
  assert_eq!(feast("cat", "yogurt"), false);
  assert_eq!(feast("electric eel", "lasagna"), false);
  assert_eq!(feast("slow loris", "salsas"), true);
  assert_eq!(feast("ox", "orange lox"), true);
  assert_eq!(feast("blue-footed booby", "blueberry"), true);
}
#[test]
fn random_test() {
    fn solution(beast: &str, dish: &str) -> bool {
        // 标记：&[..]
        &beast[..1] == &dish[..1] && &beast[beast.len()-1..] == &dish[dish.len()-1..]
    }
    
    for _ in 0..10 {
        let start = make_string(1);
        let end = make_string(1);
        let mut beast = String::new();
        let mut dish = String::new();
        
        beast.push_str(&start);
        beast.push_str(&make_string(rand::thread_rng().gen_range(5..10)));
        beast.push_str(&end);
        
        dish.push_str(&start);
        dish.push_str(&make_string(rand::thread_rng().gen_range(5..10)));
        dish.push_str(&end);
        assert_eq!(feast(&beast, &dish), solution(&beast, &dish));
    }
}