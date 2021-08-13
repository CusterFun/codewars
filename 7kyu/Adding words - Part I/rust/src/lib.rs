struct Arith{
	value : &'static str,
}

impl Arith {
    fn add(&self, str: &str) -> & str {
        let n = str_to_num(self.value) + str_to_num(str);
        num_to_str(n)
    }
}

fn str_to_num(s: &str) -> u64 {
    match s {
        "zero" => 0,
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        "ten" => 10,
        _ => panic!(),
    }
}
fn num_to_str(n: u64) -> &'static str {
    match n {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        10 => "ten",
        11 => "eleven",
        12 => "twelve",
        13 => "thirteen",
        14 => "fourteen",
        15 => "fiveteen",
        16 => "sixteen",
        17 => "seventeen",
        18 => "eighteen",
        19 => "nineteen",
        20 => "twenty",
        _ => panic!(),
    }
}

#[test]
fn returns_expected() {
  let c = Arith{value: "three"};
  assert_eq!(c.add("seven"), "ten");
  assert_eq!(c.add("eight"), "eleven");
  assert_eq!(c.add("zero"), "three");
}

#[test]
fn returns_random() {
  //Still no way to add [dependencies] rand="0.3.0" to Codewars Cargo.toml
  //So it's not truly random
  let word = vec!["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten"];
  for i in 0..11{
    let c = Arith{value: word[i]};
    let d = ArithRTY{value: word[i]};
    println!("{} + {}", word[i], word[10-i]);
    assert_eq!(c.add(word[10-i]), d.add(word[10-i]));
  }
}

struct ArithRTY{
    value : &'static str,
}

impl ArithRTY{
    fn add(&self, w:&str)->&str{
        let word = vec!["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
            "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen", "twenty"];
        word[self.w_dict(self.value)+self.w_dict(w)]
    }
    
    fn w_dict(&self, w : &str)-> usize{
        let word = vec!["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten"];
        word.iter().position(|&x| x == w).unwrap()
    }
}