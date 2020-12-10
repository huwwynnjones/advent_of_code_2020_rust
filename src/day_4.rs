use std::{
    collections::HashSet,
    fs::File,
    io,
    io::{BufRead, BufReader},
};

fn read_passport_data(input: &[String]) -> Vec<String> {
    input
        .iter()
        .flat_map(|line| line.split_whitespace())
        .map(|i| i.split_terminator(':').collect::<Vec<&str>>())
        .map(|kv| kv.first().expect("Missing key/value").to_string())
        .collect()
}

fn valid_passport_data(input: &[String]) -> bool {
    let mut valid_keys = HashSet::new();
    valid_keys.insert("ecl");
    valid_keys.insert("pid");
    valid_keys.insert("eyr");
    valid_keys.insert("hcl");
    valid_keys.insert("byr");
    valid_keys.insert("iyr");
    valid_keys.insert("hgt");

    let passport_keys = input.iter().map(|s| s.as_ref()).collect::<HashSet<&str>>();

    passport_keys.is_superset(&valid_keys)
}

pub fn load_input_file(file_name: &str) -> io::Result<Vec<Vec<String>>> {
    let input = File::open(file_name)?;
    let reader = BufReader::new(input);
    let mut passports = Vec::new();
    let mut passport = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(l) => {
                if l.is_empty() {
                    passports.push(passport.clone());
                    passport.clear();
                } else {
                    passport.push(l)
                }
            }
            Err(err) => panic!("No line to read"),
        }
    }
    passports.push(passport);
    Ok(passports)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_read_passport_data() {
        let passport_data = Vec::from([
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd".to_string(),
            "byr:1937 iyr:2017 cid:147 hgt:183cm".to_string(),
        ]);
        let correct_output: Vec<String> = Vec::from([
            "ecl".into(),
            "pid".into(),
            "eyr".into(),
            "hcl".into(),
            "byr".into(),
            "iyr".into(),
            "cid".into(),
            "hgt".into(),
        ]);
        assert_eq!(read_passport_data(&passport_data), correct_output)
    }

    #[test]
    fn test_valid_passport_data() {
        let passport_data: Vec<String> = Vec::from([
            "ecl".into(),
            "pid".into(),
            "eyr".into(),
            "hcl".into(),
            "byr".into(),
            "iyr".into(),
            "cid".into(),
            "hgt".into(),
        ]);
        assert_eq!(valid_passport_data(&passport_data), true)
    }

    #[test]
    fn test_valid_passport_data_invalid_keys() {
        let passport_data: Vec<String> = Vec::from([
            "ecl".into(),
            "pid".into(),
            "eyr".into(),
            "hcl".into(),
            "byr".into(),
            "iyr".into(),
            "cid".into(),
        ]);
        assert_eq!(valid_passport_data(&passport_data), false)
    }

    #[test]
    fn test_valid_passport_data_missing_optional_keys() {
        let passport_data: Vec<String> = Vec::from([
            "ecl".into(),
            "pid".into(),
            "eyr".into(),
            "hcl".into(),
            "byr".into(),
            "iyr".into(),
            "hgt".into(),
        ]);
        assert_eq!(valid_passport_data(&passport_data), true)
    }

    #[test]
    fn test_load_input_file() {
        let input = load_input_file("day_4_test.txt").expect("Unable to load the file");
        let correct_list = Vec::from([
            [
                "hcl:#6b5442 ecl:brn iyr:2019".to_string(),
                "pid:637485594 hgt:171cm".to_string(),
                "eyr:2021 byr:1986".to_string(),
            ],
            [
                "eyr:2025 iyr:1938 byr:2014 hcl:#341e13".to_string(),
                "hgt:66cm".to_string(),
                "pid:70195175".to_string(),
            ],
        ]);
        assert_eq!(input, correct_list)
    }

    //#[test]
    //fn count_valid_passports(){
    //    let passport_data = Vec::from([
    //        [
    //           "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd".to_string(),
    //            "byr:1937 iyr:2017 cid:147 hgt:183cm".to_string()
    //        ],
    //        [
    //            "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884".to_string(),
    //             "hcl:#cfa07d byr:1929".to_string()
    //         ],
    //         [
    //             "hcl:#ae17e1 iyr:2013".to_string(),
    //             "eyr:2024".to_string(),
    //             "ecl:brn pid:760753108 byr:1931".to_string(),
    //             "hgt:179cm".to_string();
    //         ],
    //     ]);
    // }
}
