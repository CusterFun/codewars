fn update_light(current: &str) -> String {
    // Code goes here..
    match current {
        "green" => "yellow",
        "yellow" => "red",
        "red" => "green",
        _ => panic!()
    } .into()
}

use rand::Rng;

#[test]
fn more_test() {
  assert_eq!(update_light("green"), "yellow");
  assert_eq!(update_light("yellow"), "red");
  assert_eq!(update_light("red"), "green");
}

#[test]
fn random_test() {
    // 标记：list 求余
    let lights = ["green", "yellow", "red"];
    let mut rng = rand::thread_rng();
    for _ in 0..100 {
        let r = rng.gen_range(0..3);
        assert_eq!(update_light(lights[r]), lights[(r+1)%3]);
    }   
}