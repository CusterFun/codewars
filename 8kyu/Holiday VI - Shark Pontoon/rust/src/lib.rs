fn shark(pontoon_distance: f64, shark_distance: f64, you_speed: f64, shark_speed: f64, dolphin: bool) -> String {
    // todo!()
    let you_second = pontoon_distance / you_speed;
    let shark_second = shark_distance / shark_speed;
    if dolphin {
        if you_second < shark_second * 2.0 {
            return "Alive!".to_string();
        } else {
            return "Shark Bait!".to_string();
        }
    } else {
        if you_second < shark_second {
            return "Alive!".to_string();
        } else {
            return "Shark Bait!".to_string();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(shark(12.0, 50.0, 4.0, 8.0, true), "Alive!");
        assert_eq!(shark(7.0, 55.0, 4.0, 16.0, true), "Alive!");
        assert_eq!(shark(24.0, 0.0, 4.0, 8.0, true), "Shark Bait!");
        assert_eq!(shark(40.0, 35.0, 3.0, 20.0, true), "Shark Bait!");
        assert_eq!(shark(7.0, 8.0, 3.0, 4.0, true), "Alive!");
    }
    
    use rand::Rng;
    
    #[test]
    fn random() {
        let mut rng = rand::thread_rng();
        
        for _ in 0..100 {
            let pontoon_distance = rng.gen_range(1.0..51.0);
            let shark_distance = rng.gen_range(0.0..201.0);
            let you_speed = rng.gen_range(0.0..5.0);
            let shark_speed = rng.gen_range(1.0..21.0);
            let dolphin = rng.gen_ratio(1, 2);
            
            assert_eq!(
                shark(pontoon_distance, shark_distance, you_speed, shark_speed, dolphin),
                shark_solution(pontoon_distance, shark_distance, you_speed, shark_speed, dolphin),
            );
        }
    }
    
    fn shark_solution(human_dist: f64, mut shark_dist: f64, human_speed: f64, shark_speed: f64, dolphin: bool) -> String {
        if dolphin { shark_dist *= 2.0 }
        if human_dist / human_speed < shark_dist / shark_speed { "Alive!" } else { "Shark Bait!" }.to_owned()
    }    
}