#[cfg(test)]
mod test {

    #[test]
    fn test_parse_password_policy() {
        let correct_policy = Policy::new(1, 3, 'a');
        let input = "1-3 a";
        assert_eq!(correct_policy, parse_password_policy(&input))
    }
}
