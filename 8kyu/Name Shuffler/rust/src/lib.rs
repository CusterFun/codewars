fn name_shuffler(s: &str) -> String {
    // todo!()
    let str = s.to_string();
    let mut res = str.split(" ").collect::<Vec<&str>>();
    res.swap(0, 1);
    res.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_basic() {
        assert_eq!(name_shuffler_rsplit("john McClane"), "McClane john");
        assert_eq!(name_shuffler_reverse("Mary jeggins"), "jeggins Mary");
        assert_eq!(name_shuffler_rev("tom jerry"), "jerry tom");
    }
    
    #[test]
    fn test_extended() {
        assert_eq!(name_shuffler("john McClane"),"McClane john");
        assert_eq!(name_shuffler("Mary jeggins"),"jeggins Mary");
        assert_eq!(name_shuffler("tom jerry"),"jerry tom");
        assert_eq!(name_shuffler("Mary Jane"),"Jane Mary");
        assert_eq!(name_shuffler("John Doe"),"Doe John");
        assert_eq!(name_shuffler("William O\'Brien"),"O\'Brien William");
        assert_eq!(name_shuffler("George Huffingquane-McGafferty"),"Huffingquane-McGafferty George");
    }
    
    #[test]
    fn test_randomized() {
        let first_names=vec!["Augustus","Tobias","Vernon","Ryan","Bob","Kareem","Miguel","Cyril","Chris","Simon","Tim"];
        let last_names=vec!["Hill","Beecher","Schillinger","O'Reily","Rebadow","Said","Alvarez","O'Reily","Keller","Adebisi","McManus"];

        let mut rng = rand::thread_rng();
        for _ in 1..40 {
            let first_name = first_names[rng.gen_range(0..first_names.len())];
            let last_name = last_names[rng.gen_range(0..first_names.len())];
            assert_eq!(name_shuffler(&format!("{} {}", first_name, last_name)),
                format!("{} {}", last_name, first_name));
        }
    }

    // 标记：rsplit 使用
    fn name_shuffler_rsplit(s: &str) -> String {
        s.rsplit(" ").collect::<Vec<&str>>().join(" ")

    }

    // 标记： 字符串数组 reverse 用法
    fn name_shuffler_reverse(s: &str) -> String {
        let mut r = s.split_whitespace().collect::<Vec<&str>>();
        r.reverse();
        r.join(" ")
    }

    // 标记：split(' ') 和 数组 rev 使用
    fn name_shuffler_rev(s: &str) -> String {
        return s.split(' ').rev().collect::<Vec<&str>>().join(" ");
    }
}