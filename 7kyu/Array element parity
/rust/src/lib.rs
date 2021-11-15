/// 在这个 "卡塔 "中，你将得到一个整数数组，其元素既有负值也有正值，
/// 只有一个整数只有负值或只有正值。你的任务是找到那个整数

fn solve(arr: &Vec<i32>) -> i32 {
    for i in 0..arr.len() {
        let mut flag = false;
        for j in 0..arr.len() {
            if arr[j] == arr[i] * -1 {
                flag = true;
                break;
            }
        }
        if !flag {
            return arr[i];
        }
    }
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn sample_tests() {
        assert_eq!(solve(&vec![1, -1, 2, -2, 3]), 3);
        assert_eq!(solve(&vec![-3, 1, 2, 3, -1, -4, -2]), -4);
        assert_eq!(solve(&vec![1, -1, 2, -2, 3, 3]), 3);
        assert_eq!(
            solve(&vec![-110, 110, -38, -38, -62, 62, -38, -38, -38]),
            -38
        );
        assert_eq!(best_solve(&vec![-9, -105, -9, -9, -9, -9, 105]), -9);
    }

    fn best_solve(xs: &Vec<i32>) -> i32 {
        use itertools::Itertools;
        xs.iter().unique().sum()
    }

    #[test]
    fn random_tests() {
        use rand::seq::SliceRandom;
        use rand::thread_rng;
        use rand::Rng;
        use std::collections::HashSet;

        let mut rng = thread_rng();

        // do 15 random tests
        for _ in 0..15 {
            let random_pair_count = rng.gen_range(50..151);
            let correct_answer = rng.gen_range(1..10000000);
            let correct_answer_duplicates = rng.gen_range(0..10);

            let mut bases = HashSet::with_capacity(random_pair_count);

            // generate some unique pairs
            for _ in 0..random_pair_count {
                let base = rng.gen_range(1..10000000);
                bases.insert(base);
            }
            // make sure unique pairs don't contain the correct answer
            bases.remove(&correct_answer);

            // collect the random unique pairs
            let mut input = Vec::with_capacity(1 + correct_answer_duplicates + 2 * bases.len());
            for base in bases {
                input.push(base);
                input.push(-base);
            }

            // add correct answer
            input.push(correct_answer);

            // add duplicates of correct answer (if correct_answer_duplicates > 0)
            for _ in 0..correct_answer_duplicates {
                input.push(correct_answer);
            }

            // shuffle all pairs and correct answers
            input.shuffle(&mut rng);

            // assert solve output
            assert_eq!(solve(&input), correct_answer);
        }
    }
}
