use std::iter::FromIterator;

fn hello(name: &str) -> String {
    // unimplemented!();
    let mut str = String::new();
    if name == "" {
        str = String::from("World!");
    } else {
        let arr: Vec<char> = name.to_lowercase().chars().collect();
        let c = &arr[0].to_uppercase();
        str = format!("{}{}!", c, String::from_iter(&arr[1..]));
    }
    return format!("Hello, {}", str);
}
// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(hello("johN"), "Hello, John!");
        assert_eq!(hello("alice"), "Hello, Alice!");
        assert_eq!(hello(""), "Hello, World!");
    }
    
    use rand::{thread_rng, Rng, seq::SliceRandom};
    
    #[test]
    fn random() {
        #[rustfmt::skip]
        let names = [
            "Jose", "Karim", "Fatima", "Nadia", "Mariam", "Zineb",
            "Nikita", "Nare", "Sofia", "Mariya", "Idris",
            "Pedro", "Mateo", "Miguel", "Carlos", "James", "Thomas",
            "Ahmed", "Mohammed", "Ali", "Said", "Omar", "Juan",
            "Maria", "Brianna", "Feng", "Wei", "Yi", "David",
            "Sara", "Lucas", "Nikola", "Alexander", "Dragan",
            "Anna", "Mariami", "Ana", "Nikau", "Ataahua"
        ];
        
        let mut rng = thread_rng();
        
        for _ in 0..42 {
            let name = names
                .choose(&mut rng)
                .unwrap()
                .chars()
                .map(|c| match rng.gen_ratio(1, 2) {
                    true => c.to_uppercase().collect::<String>(),
                    false => c.to_lowercase().collect::<String>(),
                })
                .collect::<String>();
            
            assert_eq!(hello(&name), hello_solution(&name));
            assert_eq!(hello_best(&name), hello_solution(&name));
        }
    }
    
    // 标记：字符串 to_uppercase 用法 flatten 用法
    fn hello_solution(name: &str) -> String {
        if name.is_empty() {
            return String::from("Hello, World!");
        }
    
        let initial = name
            .chars()
            .nth(0)
            .unwrap()
            .to_uppercase()
            .collect::<String>();
        let remainder = name
            .chars()
            .skip(1)
            .map(char::to_lowercase)
            .flatten()
            .collect::<String>();
    
        format!("Hello, {}!", initial + &remainder)
    }

    fn hello_best(name: &str) -> String {
        if name.is_empty() {
            return String::from("Hello, World!");
        }
        return format!("Hello, {}!",
            name[..1].to_uppercase() + &name[1..].to_lowercase()
        );
    }
}