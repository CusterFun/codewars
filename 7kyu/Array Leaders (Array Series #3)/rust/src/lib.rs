/// 如果一个元素大于其右边的所有元素的总和，那么它就是 array leader。

fn array_leaders(arr: &[i32]) -> Vec<i32> {
    let mut leaders = vec![];
    for i in 0..arr.len() {
        let mut sum = 0;
        for j in i + 1..arr.len() {
            sum += arr[j];
        }
        if sum < arr[i] {
            leaders.push(arr[i]);
        }
    }
    leaders
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        // Positive values
        assert_eq!(array_leaders(&[1, 2, 3, 4, 0]), [4]);
        assert_eq!(array_leaders(&[16, 17, 4, 3, 5, 2]), [17, 5, 2]);

        // Negative values
        assert_eq!(array_leaders(&[-1, -29, -26, -2]), [-1]);
        assert_eq!(array_leaders(&[-36, -12, -27]), [-36, -12]);

        // Mixed values
        assert_eq!(array_leaders(&[5, -2, 2]), [5, 2]);
        assert_eq!(best_array_leaders(&[0, -1, -29, 3, 2]), [0, -1, 3, 2]);
    }

    fn best_array_leaders(arr: &[i32]) -> Vec<i32> {
        let mut ans = Vec::new();
        for i in 1..=arr.len() {
            if arr[i - 1] > arr.iter().skip(i).sum::<i32>() {
                ans.push(arr[i - 1]);
            }
        }
        ans
    }

    use rand::Rng;

    #[test]
    fn random() {
        let mut rng = rand::thread_rng();

        for _ in 0..100 {
            let xs = (0..rng.gen_range(3..204))
                .map(|_| {
                    let min = rng.gen_range(0..1_000);
                    let max = rng.gen_range(0..1_000);

                    max - min
                })
                .collect::<Vec<_>>();

            assert_eq!(array_leaders(&xs[..]), array_leaders_solution(&xs[..]));
        }
    }

    fn array_leaders_solution(xs: &[i32]) -> Vec<i32> {
        let sums_after = {
            let mut sum = 0;
            let mut res = vec![0; xs.len()];

            for (i, x) in xs.iter().enumerate().rev() {
                res[i] = sum;
                sum += x;
            }

            res
        };

        xs.iter()
            .zip(sums_after.iter())
            .filter_map(|(a, b)| if a > b { Some(*a) } else { None })
            .collect()
    }
}
