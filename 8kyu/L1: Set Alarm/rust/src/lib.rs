fn set_alarm(employed: bool, vacation: bool) -> bool {
    // todo!();
    if employed == true && vacation == false {
        return true;
    } else {
        return false;
    }
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::set_alarm;

    #[test]
    fn test_set_alarm() {
        assert_eq!(set_alarm(true, true), false, "Fails when input is true, true");
        assert_eq!(set_alarm(false, true), false, "Fails when input is false, true");
        assert_eq!(set_alarm(false, false), false, "Fails when input is false, false");
        assert_eq!(set_alarm(true, false), true, "Fails when input is true, false");
    }

    fn set_alarm_best(employed: bool, vacation: bool) -> bool {
        employed && !vacation
    }
}


