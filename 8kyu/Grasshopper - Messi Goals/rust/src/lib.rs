static la_liga_goals: u32 = 43;
static champions_league_goals: u32 = 10;
static copa_del_rey_goals: u32 = 5;

static total_goals: u32 = la_liga_goals + champions_league_goals + copa_del_rey_goals;

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(la_liga_goals, 43,"la_liga_goals should equal to 43");
        assert_eq!(champions_league_goals, 10, "champions_league_goals should equal to 10");
        assert_eq!(copa_del_rey_goals, 5, "copa_del_rey_goals should equal to 5");
        assert_eq!(total_goals, 58, "total goals should equal to 58");
    }
}