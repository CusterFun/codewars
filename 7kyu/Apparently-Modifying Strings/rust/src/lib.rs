fn apparently(string: &str) -> String {
    let mut result = String::new();
    let str = string.split_whitespace().collect::<Vec<&str>>();

    for (i, c) in str.iter().enumerate() {
        result.push_str(c);
        if i != str.len() - 1 {
            result.push_str(" ");
        }

        if *c == "and" && i + 1 < str.len() && str[i+1] != "apparently" {
            result.push_str("apparently ");
        } else if *c == "but" && i + 1 < str.len() && str[i+1] != "apparently" {
            result.push_str("apparently ");
        } else if (*c == "and" || *c == "but") && i + 1 == str.len() {
            result.push_str(" apparently");
        }
    }
    result
}

/// Test solution on given string
fn test_exp(a: &str, exp: &str) {
    assert_eq!(best_apparently(a), exp.to_string());
}

/// Expected result test
#[test]
fn test_apparently() {
    test_exp("A fast-food resteraunt down the street was grumbling my tummy but I could not go.", "A fast-food resteraunt down the street was grumbling my tummy but apparently I could not go.");
    test_exp("apparently", "apparently");
    test_exp("and", "and apparently");
    test_exp("but", "but apparently");
    test_exp("but apparently", "but apparently");
    test_exp("and apparently", "and apparently");
    test_exp("but but but and and and", "but apparently but apparently but apparently and apparently and apparently and apparently");
    test_exp("", "");
    test_exp("but and apparently apparently apparently apparently", "but apparently and apparently apparently apparently apparently");
    test_exp("and apparentlybutactuallynot voilewtfman", "and apparently apparentlybutactuallynot voilewtfman");
    test_exp("and unapparently", "and apparently unapparently");
    test_exp("but apparentlx and apparentlx", "but apparently apparentlx and apparently apparentlx");
    test_exp("but the bread and butter apparently brand apparently", "but apparently the bread and apparently butter apparently brand apparently");
}

/// Author's solution
fn solved_apparently(string: &str) -> String {
    if string.len() == 0 || (!string.contains("and") && !string.contains("but")) { return string.to_string(); }
    string.split(' ')
        .zip(string.split(' ').skip(1).chain(::std::iter::once("")))
        .map(|p| match p {
            (any, "apparently") => any,
            ("but", _) => "but apparently",
            ("and", _) => "and apparently",
            (any, _) => any
        })
        .collect::<Vec<_>>().join(" ")
}

use rand::{Rng, thread_rng};

/// Generate a random sentence to test
fn random_sentence() -> String {
    let mut g = thread_rng();
    let words = vec!["and", "but", "apparently"];
    (0..g.gen_range(1..6)).map(|_| {
        let mut s = words[g.gen_range(0..words.len())].to_string(); s.push(' ');
        for _ in 0..g.gen_range(2..10) {
            s.push(g.gen_range(97..122) as u8 as char);
        } s
    })
    .collect::<Vec<_>>().join(" ")
}

/// Randomly generated test
#[test]
fn test_random_apparently() {
    for _ in 0..37 {
        let s = random_sentence();
        test_exp(&s, &solved_apparently(&s));
    }
}

fn best_apparently(string: &str) -> String {
    let words = string.split_whitespace().collect::<Vec<_>>();
    (0..words.len()).map(|i| match words[i] {
        "and" => if i + 1 == words.len() || words[i+1] != "apparently" {"and apparently"} else {"and"},
        "but" => if i + 1 == words.len() || words[i+1] != "apparently" {"but apparently"} else {"but"},
        s => s
    }).collect::<Vec<_>>().join(" ")
}
