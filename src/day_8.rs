#[derive(Debug, Eq, PartialEq)]
enum OpCode {
    Nop,
}

impl From<&&str> for OpCode {
    fn from(s: &&str) -> Self {
        match *s {
            "nop" => OpCode::Nop,
            _ => panic!("uknown string for opcode"),
        }
    }
}

fn string_to_op_pair(input: &str) -> (OpCode, i64) {
    let string_list = input.split_whitespace().collect::<Vec<&str>>();
    (
        string_list.first().unwrap().into(),
        string_list.last().unwrap().parse::<i64>().unwrap(),
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_string_to_op_pair() {
        assert_eq!(string_to_op_pair("nop +0"), (OpCode::Nop, 0))
    }
}
