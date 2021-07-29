fn century(year: u32) -> u32 {
    (year + 99) / 100
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
    assert_eq!(century(1705), 18);
    assert_eq!(century(1900), 19);
    assert_eq!(century(1601), 17);
    assert_eq!(century(2000), 20);
    assert_eq!(century(89), 1);
    assert_eq!(century(101), 2);
    }
}
