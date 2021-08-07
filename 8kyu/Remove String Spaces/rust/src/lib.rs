fn no_space(x : String) -> String{
  x.to_string().split_whitespace().collect::<Vec<_>>().join("")
}

use rand::Rng;

#[test]
fn returns_expected() {
  assert_eq!("8j8mBliB8gimjB8B8jlB", no_space("8 j 8   mBliB8g  imjB8B8  jl  B".to_string()));
  assert_eq!("88Bifk8hB8BB8BBBB888chl8BhBfd", no_space("8 8 Bi fk8h B 8 BB8B B B  B888 c hl8 BhB fd".to_string()));
  assert_eq!("8aaaaaddddr", no_space_split_whitspace("8aaaaa dddd r     ".to_string()));
  assert_eq!("jfBmgklf8hg88lbe8", no_space("jfBm  gk lf8hg  88lbe8 ".to_string()));
  assert_eq!("8jaam", no_space("8j aam".to_string()));
}

fn random_string() -> String {
    let mut rng = rand::thread_rng();
    let letters = "      abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890            ";
    vec![' '; rng.gen_range(2..19)]
        .iter()
        .map(|_| letters.chars().nth(rng.gen_range(0..letters.len())).unwrap())
        .collect::<String>()
}

#[test]
fn random_tests() {
    for _ in 0..100 {
        let s = random_string();
        // 标记：replace 使用
        assert_eq!(no_space(s.clone()), s.replace(" ", ""));
    }
}

fn no_space_split_whitspace(x : String) -> String{
    x.split_whitespace().collect()
}