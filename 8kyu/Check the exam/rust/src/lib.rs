fn check_exam(arr_a: &[&str], arr_b: &[&str]) -> i64 {
    let mut res: i64 = 0;
    for i in 0..arr_a.len() {
        if arr_b[i].is_empty() {
            continue; // 学生的空白答案+0
        } else if arr_b[i] == arr_a[i] {
            res += 4; // 一样的答案+4
        } else if arr_b[i] != arr_a[i] {
            res -= 1; // 不一样的答案-1
        }
    }
    if res < 0 {
        return 0;
    }
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(check_exam(&["a", "a", "b", "b"], &["a", "c", "b", "d"]), 6);
        assert_eq!(check_exam(&["a", "a", "c", "b"], &["a", "a", "b",  ""]), 7);
        assert_eq!(check_exam(&["a", "a", "b", "c"], &["a", "a", "b", "c"]), 16);
        assert_eq!(check_exam(&["b", "c", "b", "a"], &["",  "a", "a", "c"]), 0);
    }
    
    use rand::Rng;
    
    const ANSWERS_CORRECT: [&str; 4] = ["a", "b", "c", "d"];
    const ANSWERS_OPTIONS: [&str; 5] = ["a", "b", "c", "d", ""];
    
    #[test]
    fn random() {
        let mut rng = rand::thread_rng();
        
        for _ in 0..100 {
            let mut xs = vec![];
            let mut ys = vec![];
            
            for _ in 0..rng.gen_range(4..21) {
                xs.push(ANSWERS_CORRECT[rng.gen_range(0..4)]);
                ys.push(ANSWERS_OPTIONS[rng.gen_range(0..5)]);
            }

            assert_eq!(
                check_exam(&xs[..], &ys[..]),
                check_exam_solution(&xs[..], &ys[..]),
            );
        }
    }
    
    fn check_exam_solution(xs: &[&str], ys: &[&str]) -> i64 {
        xs.iter().zip(ys.iter()).fold(0, |pts, (a, b)| pts + if b.is_empty() { 0 } else if a == b { 4 } else { -1 }).max(0)
    }    
}