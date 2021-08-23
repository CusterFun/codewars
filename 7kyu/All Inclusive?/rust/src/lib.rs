fn contain_all_rots(strng: &str, arr: Vec<&str>) -> bool {
    for i in 0..strng.len() {
        let temp = format!("{}{}", &strng[i..], &strng[..i]);
        if !arr.contains(&temp.as_str()) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(strng: &str, arr: Vec<&str>, exp: bool) -> () {
        println!("strng: {}", strng);
        println!("arr: {:?}", arr);
        let ans = contain_all_rots(strng, arr);
        println!("actual:\n{}", ans);
        println!("expect:\n{}", exp);
        println!("{}", ans == exp);
        assert_eq!(ans, exp);
        println!("{}", "-");
    }

    #[test]
    fn basis_tests() {
        dotest("", vec![], true);
        dotest("", vec!["bsjq", "qbsj"], true);
        dotest("bsjq", vec!["bsjq", "qbsj", "sjqb", "twZNsslC", "jqbs"], true);
        dotest("XjYABhR", vec!["TzYxlgfnhf", "yqVAuoLjMLy", "BhRXjYA", "YABhRXj", "hRXjYAB", "jYABhRX", "XjYABhR", "ABhRXjY"], false);
        dotest("QJAhQmS", vec!["hQmSQJA", "QJAhQmS", "QmSQJAh", "yUgUXoQE", "AhQmSQJ", "mSQJAhQ", "SQJAhQm", "JAhQmSQ"], true);
        dotest("Etsshp", vec!["nVOETcaxdvuk", "shpEts", "hpEtss", "Etsshp", "OuIiQ", "sXrDdPXIaW", "tsshpE", "pEtssh"], false);
        dotest("Ajylvpy", vec!["Ajylvpy", "ylvpyAj", "jylvpyA", "lvpyAjy", "pyAjylv", "vpyAjyl"], false);
        dotest("MqhWvHF", vec!["numMfygcH", "HFMqhWv", "qhWvHFM", "ZJKKxM", "hWvHFMq", "MqhWvHF", "hfZWYSqk", "BTcSoEdchPlL", "WvHFMqh", "vHFMqhW", "FMqhWvH"], true);
        dotest("UDvG", vec!["vGUD", "UDvG", "GUDv", "DvGU"], true);
        dotest("sObPfw", vec!["ObPfws", "Cofuhqrmmzq", "wFvfcqU", "sObPfw", "bPfwsO", "PfwsOb", "wsObPf", "fwsObP"], true);
        dotest("KUckM", vec!["MKUck", "EDjfbQB", "GUPwzk", "SKZvilwnL", "UckMK", "KUckM", "kMKUc"], false);
        dotest("FDIe", vec!["DIeF", "IeFD", "FDIe", "eFDI"], true);
        dotest("12341234", vec!["DIeF", "IeFD", "12341234", "41234123", "34123412", "23412341"], true);
    }

    extern crate rand;
    use self::rand::{thread_rng, Rng};
    use self::rand::distributions::Alphanumeric;
    use self::rand::prelude::SliceRandom;

    fn dostr(a: i32) -> String {
        let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(a as usize)
            .map(char::from)
            .collect();
        return rand_string;
    }

    fn contain_all_rots_pk(strng: &str, arr: Vec<&str>) -> bool { 
        fn all_rots(s: &str) -> Vec<String> {
            let mut res: Vec<String> = vec![s.to_string()];
            for i in 0..s.len() {
                let u = s[i..s.len()].to_string() + &s[0..i];
                if !res.contains(&u) {
                    res.push(format!("{}", u));
                }
            }
            return res;
        }
        if strng.len() == 0 {return true;}    
        let rots = all_rots(strng);
        for el in rots {
            if arr.contains(&&el[..]) == false {
                return false;
            }
        }
        return true;
    }

    #[test]
    fn random_tests() {
        fn all_rots(s: &str) -> Vec<String> {
            let mut res: Vec<String> = vec![s.to_string()];
            for i in 0..s.len() {
                let u = s[i..s.len()].to_string() + &s[0..i];
                if !res.contains(&u) {
                    res.push(format!("{}", u));
                }
            }
            return res;
        }
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let s = &dostr(rng.gen_range(8..15));
            let mut arr: Vec<String> = all_rots(s);
            let ss = dostr(s.len() as i32);
            arr.push(ss);
            arr.shuffle(&mut thread_rng());
            let mut narr: Vec<&str> = vec![];
            for i in 1..arr.len() {
                narr.push(&arr[i]);
            }
            let sol = contain_all_rots_pk(s, narr);
            let mut nnarr: Vec<&str> = vec![];
            for i in 1..arr.len() {
                nnarr.push(&arr[i]);
            }
            dotest(s, nnarr, sol);
        }
    }
}