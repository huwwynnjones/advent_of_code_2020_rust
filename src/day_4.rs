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
}
