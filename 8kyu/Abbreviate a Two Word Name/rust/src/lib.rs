fn abbrev_name(name: &str) -> String {
    let mut res = String::new();
    let mut v: Vec<char> = name.chars().collect();
    v[0] = v[0].to_uppercase().nth(0).unwrap();
    for (i, c) in v.iter().enumerate() {
        if *c == ' ' {
            res += &format!("{}.{}", v[0], v[i+1].to_uppercase().nth(0).unwrap())
        }
    }
    res
}

// Rust test example:
extern crate rand;
#[allow(unused_imports)]
use self::rand::Rng;

#[test]
fn sample_tests() {
  assert_eq!(abbrev_name("Sam Harris"), "S.H");
  assert_eq!(abbrev_name("Patrick Feenan"), "P.F");
  assert_eq!(abbrev_name("Evan Cole"), "E.C");
  assert_eq!(abbrev_name("P Favuzzi"), "P.F");
  assert_eq!(abbrev_name("David Mendieta"), "D.M");
}

#[test]
fn more_tests() {
  assert_eq!(abbrev_name("george clooney"), "G.C");
  assert_eq!(abbrev_name("marky mark"), "M.M");
  assert_eq!(abbrev_name("eliza doolittle"), "E.D");
  assert_eq!(abbrev_name("reese witherspoon"), "R.W");
}

#[test]
fn random_tests() {
    let names = ["Sam x", "Harris yellow", "Patrick z", "Feenan Evan", "Cole P", "Favuzzi Greg", "David Lendieta", "george sneeze", "Kile clooney", "marky bark"];
    for _ in 0..10 {
        let name = names[rand::thread_rng().gen_range(0..names.len() - 1)];
        assert_eq!(abbrev_name(&name), solution(&name));
    }
}

fn solution(name: &str) -> String {
    let mut result = String::new();
    let arr: Vec<&str> = name.split_whitespace().collect();

    result.push_str(&arr[0][..1].to_uppercase());
    result.push_str(".");
    result.push_str(&arr[1][..1].to_uppercase());

    result
}

fn abbrev_name_best(name: &str) -> String {
    name.split(' ')
        .map(|x| x.chars().nth(0).unwrap().to_string().to_uppercase())
        .collect::<Vec<_>>()
        .join(".")
}

fn abbrev_name_easy(name: &str) -> String {
    let mut names = name.split(' ');
    let first = names.next().unwrap();
    let second = names.next().unwrap();
    return first[0..1].to_uppercase().to_string() + "." + &second[0..1].to_uppercase();
}