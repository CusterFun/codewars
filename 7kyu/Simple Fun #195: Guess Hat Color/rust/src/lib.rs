mod preloaded;
use preloaded::Color;

fn guess_hat_color(a: Color, b: Color, c: Color, d: Color) -> usize {
    match b == c {
        true => 1,
        false => 2
    }
}

#[cfg(test)]
mod tests {
    use super::{guess_hat_color, preloaded::Color};

    #[test]
    fn basic() {
        use Color::*;

        assert_eq!(guess_hat_color(Black, White, Black, White), 2);
        assert_eq!(guess_hat_color(White, Black, White, Black), 2);
        assert_eq!(guess_hat_color(White, Black, Black, White), 1);
    }

    #[test]
    fn random() {
        use rand::{seq::SliceRandom, thread_rng};
        use Color::*;

        let mut rng = thread_rng();
        let mut arr = [Black, White, Black, White];

        for _ in 0..100 {
            arr.shuffle(&mut rng);

            let actual = guess_hat_color(arr[0], arr[1], arr[2], arr[3]);
            let expect = guess_hat_color_solution(arr[0], arr[1], arr[2], arr[3]);

            assert_eq!(actual, expect, "with the following hats: {:?}", arr);
        }
    }

    fn guess_hat_color_solution(_: Color, b: Color, c: Color, _: Color) -> usize {
        if b == c {
            1
        } else {
            2
        }
    }
}