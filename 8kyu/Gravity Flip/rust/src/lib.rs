fn flip(dir: char, cubes: &[u32]) -> Vec<u32> {
    let mut res = cubes.to_vec();
    res.sort();
    if dir == 'L' { res.reverse(); }
    res
}

#[cfg(test)]
mod tests {
    use super::flip;
    
    #[test]
    fn fixed_tests() {
        assert_eq!(flip('R', &vec![4, 5, 6, 7, 1]), vec![1, 4, 5, 6, 7]);
        assert_eq!(flip_itertools('L', &vec![3, 0, 9, 8, 1, 2]), vec![9, 8, 3, 2, 1, 0]);
        assert_eq!(flip_to_vec('L', &vec![0, 0, 12, 0, 1]), vec![12, 1, 0, 0, 0]);
        assert_eq!(flip('R', &vec![13, 2, 4, 7, 93]), vec![2, 4, 7, 13, 93]);
        assert_eq!(flip('R', &vec![5, 4, 3, 2, 1]), vec![1, 2, 3, 4, 5]);
    }
    
    #[test]
    fn random_tests() {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        for _ in 0..100 {
            let dir = if rng.gen::<bool>() { 'R' } else { 'L' };
            let cubes: Vec<u32> = (0..rng.gen_range(4..100))
                .map(|_| rng.gen_range(0..100))
                .collect();
            
            let expected = reference_solution(dir, &cubes);
            let actual = flip(dir, &cubes);
            assert_eq!(actual, expected);
        }
    }

    // 标记：itertools用法
    fn reference_solution(dir: char, cubes: &[u32]) -> Vec<u32> {
        use itertools::Itertools;
        if dir == 'L' {
            cubes.iter().sorted().rev().cloned().collect()
        } else {
            cubes.iter().sorted().cloned().collect()
        }
    }

    fn flip_itertools(dir: char, cubes: &[u32]) -> Vec<u32> {
        use itertools::Itertools;
        let cmp: fn(&u32, &u32) -> std::cmp::Ordering = match dir {
            'R' => |a, b| a.cmp(b),
            'L' => |a, b| b.cmp(a),
            _ => unreachable!()
        };
        cubes.iter().cloned().sorted_by(cmp).collect()
    }
    

    fn flip_to_vec(dir: char, cubes: &[u32]) -> Vec<u32> {
        let mut arr = cubes.to_vec();
        match dir {
            'R' => arr.sort_by(|a, b| a.cmp(b)),
            _ => arr.sort_by(|a, b| b.cmp(a))
        };
        arr
    }
}