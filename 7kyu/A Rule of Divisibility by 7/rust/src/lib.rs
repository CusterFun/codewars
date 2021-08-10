fn seven(n: i64) -> (i64, i32) {
    let mut n = n;
    let mut step = 0;

    while n >= 100 {
        step += 1;
        n = n / 10 - n % 10 * 2;
    }

    (n, step)
}


#[cfg(test)]
    mod tests {
    use super::*;

    fn dotest(n: i64, exp: (i64, i32)) -> () {
        println!(" n: {:?};", n);
        let ans = seven(n);
        println!(" actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!(" {};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn basic_tests() {
        dotest(1021, (10, 2));
        dotest(477557101, (28, 7));
        dotest(477557102, (47, 7)); 
        dotest(1603, (7, 2));
        dotest(371, (35, 1));
        dotest(1369851, (0, 5));
        dotest(483, (42, 1));
        dotest(483595, (28, 4));
        dotest(0, (0, 0));
        dotest(1889584453156367, (8, 14));
    }

    extern crate rand;
    use self::rand::Rng;

    fn seven_solution(n: i64) -> (i64, i32) {
        let mut steps = 0; let mut m = n;
        while m > 99 {
            let q = m /10; let r = m % 10;
            m = q - (2 * r);
            steps += 1;
        }
        return (m, steps);
    }

    #[test]
    fn random_tests() {
        let mut rng = rand::thread_rng();
        for _ in 0..200 {
            let n = rng.gen_range(10..75000000);
            let sol = seven_solution(n);
            dotest(n, sol);
        }
    }

    // 标记：递归
    fn seven_recursion(n: i64) -> (i64, i32) {
        if n < 100 {
            return (n, 0);
        }
        let tuple:(i64, i32) = seven_recursion((n/10) - 2*(n%10));
        return (tuple.0, tuple.1+1);
    }
}