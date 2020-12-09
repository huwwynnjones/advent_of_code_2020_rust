use std::{
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
}
