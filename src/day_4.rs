use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    convert::TryInto,
    fs::File,
    io,
    io::{BufRead, BufReader},
};

fn read_passport_data(input: &[String]) -> HashMap<String, String> {
    input
        .iter()
        .flat_map(|line| line.split_whitespace())
        .map(|i| i.split_terminator(':').collect::<Vec<&str>>())
        .map(|kv| {
            (
                kv.get(0).expect("Missing key/value").to_string(),
                (kv.get(1).expect("Missing key/value").to_string()),
            )
        })
        .collect()
}

#[derive(Copy, Clone)]
pub enum CountType {
    KeysOnly,
    KeysAndValues,
}

fn valid_passport_data(input: &HashMap<String, String>, count_type: CountType) -> bool {
    let keys = input.keys().map(|s| s.as_ref()).collect::<HashSet<&str>>();
    match count_type {
        CountType::KeysOnly => passport_keys_are_valid(keys),
        CountType::KeysAndValues => {
            passport_keys_are_valid(keys) && passport_values_are_valid(input)
        }
    }
}

fn passport_keys_are_valid(passport_keys: HashSet<&str>) -> bool {
    let mut valid_keys = HashSet::new();
    valid_keys.insert("ecl");
    valid_keys.insert("pid");
    valid_keys.insert("eyr");
    valid_keys.insert("hcl");
    valid_keys.insert("byr");
    valid_keys.insert("iyr");
    valid_keys.insert("hgt");

    passport_keys.is_superset(&valid_keys)
}

fn passport_values_are_valid(passport_data: &HashMap<String, String>) -> bool {
    for key in passport_data.keys() {
        let value = passport_data.get(key).expect("No value");
        match key.as_ref() {
            "ecl" => {
                if !valid_eye_colour(value) {
                    return false;
                }
            }
            "pid" => {
                if !valid_passport_number(value) {
                    return false;
                }
            }
            "eyr" => {
                if !valid_expiration_year(value) {
                    return false;
                }
            }
            "hcl" => {
                if !valid_hair_colour(value) {
                    return false;
                }
            }
            "byr" => {
                if !valid_birth_year(value) {
                    return false;
                }
            }
            "iyr" => {
                if !valid_issue_year(value) {
                    return false;
                }
            }
            "hgt" => {
                if !valid_height(value) {
                    return false;
                }
            }
            _ => (),
        }
    }
    true
}

fn valid_birth_year(birth_year: &str) -> bool {
    valid_number(birth_year, 1920, 2002)
}

fn valid_issue_year(issue_year: &str) -> bool {
    valid_number(issue_year, 2010, 2020)
}

fn valid_expiration_year(expiration_year: &str) -> bool {
    valid_number(expiration_year, 2020, 2030)
}

fn valid_number(number: &str, min: u32, max: u32) -> bool {
    let number = match number.parse::<u32>() {
        Ok(nmb) => nmb,
        Err(_) => return false,
    };
    number >= min && number <= max
}

fn valid_height(height: &str) -> bool {
    if height.is_empty() {
        return false;
    }
    match height.get(height.len() - 2..height.len()) {
        Some(dimension) => match dimension {
            "cm" => match height.get(0..3) {
                Some(number) => valid_number(number, 150, 193),
                None => false,
            },
            "in" => match height.get(0..2) {
                Some(number) => valid_number(number, 59, 76),
                None => false,
            },
            _ => false,
        },
        None => false,
    }
}

fn valid_hair_colour(colour: &str) -> bool {
    let reg = Regex::new(r"^#([0-9]|[a-f]){6}").expect("Unable to create Regex");
    reg.is_match(colour)
}

fn valid_eye_colour(colour: &str) -> bool {
    let valid_colours = Vec::from(["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]);
    valid_colours.contains(&colour)
}

fn valid_passport_number(number: &str) -> bool {
    if number.len() == 9 {
        number.parse::<u32>().is_ok()
    } else {
        false
    }
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
            Err(_) => panic!("No line to read"),
        }
    }
    passports.push(passport);
    Ok(passports)
}

pub fn count_valid_passports(passport_data: &[Vec<String>], count_type: CountType) -> u32 {
    passport_data
        .iter()
        .filter(|p| valid_passport_data(&read_passport_data(&p), count_type))
        .count()
        .try_into()
        .expect("Can't convert usize to us32")
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

        let mut correct_output: HashMap<String, String> = HashMap::new();
        correct_output.insert("ecl".into(), "gry".into());
        correct_output.insert("pid".into(), "860033327".into());
        correct_output.insert("eyr".into(), "2020".into());
        correct_output.insert("hcl".into(), "#fffffd".into());
        correct_output.insert("byr".into(), "1937".into());
        correct_output.insert("iyr".into(), "2017".into());
        correct_output.insert("cid".into(), "147".into());
        correct_output.insert("hgt".into(), "183cm".into());

        assert_eq!(read_passport_data(&passport_data), correct_output);
    }

    #[test]
    fn test_valid_passport_data() {
        let mut passport_data: HashMap<String, String> = HashMap::new();
        passport_data.insert("ecl".into(), "gry".into());
        passport_data.insert("pid".into(), "860033327".into());
        passport_data.insert("eyr".into(), "2020".into());
        passport_data.insert("hcl".into(), "#fffffd".into());
        passport_data.insert("byr".into(), "1937".into());
        passport_data.insert("iyr".into(), "2017".into());
        passport_data.insert("cid".into(), "147".into());
        passport_data.insert("hgt".into(), "183cm".into());
        assert_eq!(
            valid_passport_data(&passport_data, CountType::KeysAndValues),
            true
        )
    }

    #[test]
    fn test_valid_passport_data_invalid_keys() {
        let mut passport_data: HashMap<String, String> = HashMap::new();
        passport_data.insert("ecl".into(), "gry".into());
        passport_data.insert("pid".into(), "860033327".into());
        passport_data.insert("eyr".into(), "2020".into());
        passport_data.insert("hcl".into(), "#fffffd".into());
        passport_data.insert("byr".into(), "1937".into());
        passport_data.insert("iyr".into(), "2017".into());
        passport_data.insert("cid".into(), "147".into());
        assert_eq!(
            valid_passport_data(&passport_data, CountType::KeysOnly),
            false
        )
    }

    #[test]
    fn test_valid_passport_data_missing_optional_keys() {
        let mut passport_data: HashMap<String, String> = HashMap::new();
        passport_data.insert("ecl".into(), "gry".into());
        passport_data.insert("pid".into(), "860033327".into());
        passport_data.insert("eyr".into(), "2020".into());
        passport_data.insert("hcl".into(), "#fffffd".into());
        passport_data.insert("byr".into(), "1937".into());
        passport_data.insert("iyr".into(), "2017".into());
        passport_data.insert("hgt".into(), "183cm".into());
        assert_eq!(
            valid_passport_data(&passport_data, CountType::KeysOnly),
            true
        )
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

    #[test]
    fn test_count_valid_passports() {
        let passport_data = Vec::from([
            Vec::from([
                "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd".to_string(),
                "byr:1937 iyr:2017 cid:147 hgt:183cm".to_string(),
            ]),
            Vec::from([
                "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884".to_string(),
                "hcl:#cfa07d byr:1929".to_string(),
            ]),
            Vec::from([
                "hcl:#ae17e1 iyr:2013".to_string(),
                "eyr:2024".to_string(),
                "ecl:brn pid:760753108 byr:1931".to_string(),
                "hgt:179cm".to_string(),
            ]),
            Vec::from([
                "hcl:#cfa07d eyr:2025 pid:166559648".to_string(),
                "iyr:2011 ecl:brn hgt:59in".to_string(),
            ]),
        ]);

        assert_eq!(
            count_valid_passports(&passport_data, CountType::KeysOnly),
            2
        )
    }

    #[test]
    fn test_passport_values_are_valid() {
        let mut passport_data: HashMap<String, String> = HashMap::new();
        passport_data.insert("ecl".into(), "grn".into());
        passport_data.insert("pid".into(), "087499704".into());
        passport_data.insert("eyr".into(), "2030".into());
        passport_data.insert("hcl".into(), "#623a2f".into());
        passport_data.insert("byr".into(), "1980".into());
        passport_data.insert("iyr".into(), "2012".into());
        passport_data.insert("hgt".into(), "74in".into());

        assert_eq!(passport_values_are_valid(&passport_data), true)
    }

    #[test]
    fn test_passport_values_are_invalid() {
        let mut passport_data: HashMap<String, String> = HashMap::new();
        passport_data.insert("ecl".into(), "amb".into());
        passport_data.insert("pid".into(), "186cm".into());
        passport_data.insert("eyr".into(), "1972".into());
        passport_data.insert("hcl".into(), "#18171d".into());
        passport_data.insert("byr".into(), "1926".into());
        passport_data.insert("iyr".into(), "2018".into());
        passport_data.insert("hgt".into(), "170".into());
        passport_data.insert("cid".into(), "100".into());

        assert_eq!(passport_values_are_valid(&passport_data), false)
    }

    #[test]
    fn test_valid_birth_year() {
        assert_eq!(valid_birth_year("2000"), true);
        assert_eq!(valid_birth_year("number"), false);
        assert_eq!(valid_birth_year("1919"), false);
        assert_eq!(valid_birth_year("2003"), false);
        assert_eq!(valid_birth_year("1920"), true);
        assert_eq!(valid_birth_year("2002"), true);
        assert_eq!(valid_birth_year("20025"), false);
        assert_eq!(valid_birth_year("200"), false);
    }

    #[test]
    fn test_valid_issue_year() {
        assert_eq!(valid_issue_year("2015"), true);
        assert_eq!(valid_issue_year("number"), false);
        assert_eq!(valid_issue_year("2009"), false);
        assert_eq!(valid_issue_year("2021"), false);
        assert_eq!(valid_issue_year("2010"), true);
        assert_eq!(valid_issue_year("2020"), true);
        assert_eq!(valid_issue_year("20025"), false);
        assert_eq!(valid_issue_year("200"), false);
    }

    #[test]
    fn test_valid_expiration_year() {
        assert_eq!(valid_expiration_year("2025"), true);
        assert_eq!(valid_expiration_year("number"), false);
        assert_eq!(valid_expiration_year("2019"), false);
        assert_eq!(valid_expiration_year("2031"), false);
        assert_eq!(valid_expiration_year("2020"), true);
        assert_eq!(valid_expiration_year("2030"), true);
        assert_eq!(valid_expiration_year("20025"), false);
        assert_eq!(valid_expiration_year("200"), false);
    }

    #[test]
    fn test_valid_height() {
        assert_eq!(valid_height("60in"), true);
        assert_eq!(valid_height("60"), false);
        assert_eq!(valid_height("190cm"), true);
        assert_eq!(valid_height("190in"), false);
        assert_eq!(valid_height("77in"), false);
        assert_eq!(valid_height("77km"), false);
        assert_eq!(valid_height(""), false);
    }

    #[test]
    fn test_valid_hair_colour() {
        assert_eq!(valid_hair_colour("#123abc"), true);
        assert_eq!(valid_hair_colour("#123abz"), false);
        assert_eq!(valid_hair_colour("123abc"), false);
        assert_eq!(valid_hair_colour("#123456"), true);
        assert_eq!(valid_hair_colour("#abcdef"), true);
    }

    #[test]
    fn test_eye_colour() {
        assert_eq!(valid_eye_colour("amb"), true);
        assert_eq!(valid_eye_colour("blu"), true);
        assert_eq!(valid_eye_colour("brn"), true);
        assert_eq!(valid_eye_colour("gry"), true);
        assert_eq!(valid_eye_colour("grn"), true);
        assert_eq!(valid_eye_colour("hzl"), true);
        assert_eq!(valid_eye_colour("oth"), true);
        assert_eq!(valid_eye_colour("zod"), false);
    }

    #[test]
    fn test_valid_passport_number() {
        assert_eq!(valid_passport_number("000000001"), true);
        assert_eq!(valid_passport_number("0123456789"), false);
        assert_eq!(valid_passport_number("abcdefghij"), false)
    }
}
