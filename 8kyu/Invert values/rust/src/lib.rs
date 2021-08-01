fn invert(values: &[i32]) -> Vec<i32> {
    // let mut res:Vec<i32> = Vec::new();
    // for i in values.iter() {
    //     res.push(i*-1);
    // }
    // res

    values.iter().map(|x| -x).collect()
}

#[cfg(test)]
mod tests {
    use super::invert;

    #[test]
    fn basic_tests() {
        assert_eq!(invert(&vec![1,2,3,4,5]), vec![-1,-2,-3,-4,-5]);
        assert_eq!(invert(&vec![1,-2,3,-4,5]), vec![-1,2,-3,4,-5]);
        assert_eq!(invert(&vec![]), vec![]);
    }
    
    #[test]
    fn random_tests() {
        use rand::distributions::Uniform;
        use rand::Rng;
        
        let mut rng = rand::thread_rng();
        let value_distribution = Uniform::from(-1000..1000);
               
        for _ in 0..500 {
            let len = (&mut rng).gen_range(0..1000);
            let values = (&mut rng).sample_iter(&value_distribution).take(len).collect::<Vec<i32>>();
            let expected = values.iter().map(|val| -*val).collect::<Vec<i32>>();
            assert_eq!(invert(&values), expected);
        }
    }
}