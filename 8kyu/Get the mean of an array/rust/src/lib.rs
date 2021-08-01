fn get_average(marks: &[f32]) -> f32 {
    // todo!()
    (marks.iter().sum::<f32>() / marks.len() as f32).floor()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_average() {
        assert_eq!(get_average(&[2., 2., 2., 2.]), 2.);
        assert_eq!(get_average(&[1., 5., 87., 45., 8., 8.]), 25.);
        assert_eq!(get_average(&[2.,5.,13.,20.,16.,16.,10.]), 11.);
        assert_eq!(get_average(&[1.,2.,15.,15.,17.,11.,12.,17.,17.,14.,13.,15.,6.,11.,8.,7.]), 11.);
    }
    
    use rand::Rng;
    
    #[test]
    fn test_get_average_random() {
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let marks: Vec<f32> = (0..10).map(|_| rng.gen_range(0.0..100.0)).collect();
            // 标记：floor用法
            assert_eq!(get_average(&marks), (marks.iter().sum::<f32>() / marks.iter().len() as f32).floor());
        }
    }
}