fn switch_it_up(n: usize) -> &'static str {
    // todo!()
    match n {
        0 => "Zero",
        1 => "One",
        2 => "Two",
        3 => "Three",
        4 => "Four",
        5 => "Five",
        6 => "Six",
        7 => "Seven",
        8 => "Eight",
        9 => "Nine",
        _ => "",
    }
}

#[cfg(test)]
mod tests {
    use super::switch_it_up;

    #[test]
    fn basic() {
        assert_eq!(switch_it_up(1), "One");
        assert_eq!(switch_it_up(2), "Two");
        assert_eq!(switch_it_up(3), "Three");
    }

    use rand::Rng;

    #[test]
    fn random() {
        let mut rng = rand::thread_rng();

        for _ in 0..100 {
            let n = rng.gen_range(0..10);

            assert_eq!(switch_it_up(n), switch_it_up_solution(n));
        }
    }

    fn switch_it_up_solution(n: usize) -> &'static str {
        ["Zero","One","Two","Three","Four","Five","Six","Seven","Eight","Nine"][n]
    }  
}