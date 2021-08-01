fn duty_free(price: i32, discount: i32, holiday_cost: i32) -> i32 {
    // add your code
    f64::floor(holiday_cost as f64 / (price as f64 * (discount as f64 * 0.01))) as i32
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn more_tests() {
        assert_eq!(duty_free(12, 50, 1000), 166);
        assert_eq!(duty_free(17, 10, 500), 294);
        assert_eq!(duty_free(24, 35, 3000), 357);
        assert_eq!(duty_free_best(1400, 35, 10000), 20);
        assert_eq!(duty_free(700, 26, 7000), 38);
    }

    fn duty_free_best(price: i32, discount: i32, holiday_cost: i32) -> i32 {
        holiday_cost * 100 / (discount * price)
    }
}