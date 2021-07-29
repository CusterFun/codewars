fn find_average(slice: &[f64]) -> f64 {
    let total:f64 = slice.iter().sum();
    if slice.len() > 0 {
        return total / slice.len() as f64;
    }
    0.0
}

#[cfg(test)]
mod tests {
    use super::find_average;
    use float_eq::assert_float_eq;

    #[test]
    fn example() {
        let input = [
            17.0, 16.0, 16.0, 16.0, 16.0, 15.0, 17.0, 17.0, 15.0, 5.0, 17.0, 17.0, 16.0,
        ];

        assert_float_eq!(
            find_average(&input),
            find_average_solution(&input),
            abs <= 1e-9, r2nd <= 4.0 * f64::EPSILON
        );
    }

    #[test]
    fn edge_cases() {
        let input = [0.0, 0.0, 0.0, 0.0, 0.0];

        assert_float_eq!(find_average(&input), 0.0, abs <= 1e-9, r2nd <= 4.0 * f64::EPSILON);
        assert_float_eq!(find_average(&[]), 0.0, abs <= 1e-9, r2nd <= 4.0 * f64::EPSILON);
    }

    use rand::{thread_rng, Rng};

    #[test]
    fn random() {
        let mut rng = thread_rng();

        for _ in 0..100 {
            let input = (0..rng.gen_range(0..100))
                .map(|_| rng.gen_range(-1000.0..1000.0))
                .collect::<Vec<_>>();

            assert_float_eq!(
                find_average(&input),
                find_average_solution(&input),
                abs <= 1e-9, r2nd <= 4.0 * f64::EPSILON
            );
        }
    }

    fn find_average_solution(slice: &[f64]) -> f64 {
        match slice.len() {
            0 => 0.0,
            _ => slice.iter().sum::<f64>() / slice.len() as f64,
        }
    }
}