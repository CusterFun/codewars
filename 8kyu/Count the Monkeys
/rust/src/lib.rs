fn monkey_count(n: i32) -> Vec<i32> {
    let mut vec: Vec<i32> = Vec::new();
    for i in 1..n+1 {
        vec.push(i)
    }
    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(monkey_count(5), vec![1, 2, 3, 4, 5]);
        assert_eq!(monkey_count(1), vec![1]);
        assert_eq!(monkey_count(0), vec![]);
        assert_eq!(monkey_count_best(12), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
    }
}

fn monkey_count_best(n: i32) -> Vec<i32> {
    (1..=n).collect()
}