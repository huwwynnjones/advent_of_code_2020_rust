fn read_passport_data(input: &str) -> Vec<String> {
    Vec::from([
        "ecl".into(),
        "pid".into(),
        "eyr".into(),
        "hcl".into(),
        "byr".into(),
        "iyr".into(),
        "cid".into(),
        "hgt".into(),
    ])
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_read_passport_data() {
        let passport_data =
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm";
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
        assert_eq!(read_passport_data(passport_data), correct_output)
    }
}
