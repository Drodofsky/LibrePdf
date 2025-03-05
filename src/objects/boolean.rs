use nom::{IResult, Parser, branch::alt, bytes::complete::tag};
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Boolean(bool);

impl Boolean {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Boolean> {
        alt((
            tag("true").map(|_| Boolean(true)),
            tag("false").map(|_| Boolean(false)),
        ))
        .parse(input)
    }
    pub fn get(&self) -> bool {
        self.0
    }
    pub fn new(b: bool) -> Self {
        Self(b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn boolean_1() {
        let (rem, parsed) = Boolean::parse(b"true").unwrap();
        assert!(rem.is_empty());
        assert_eq!(parsed, Boolean(true));
    }
    #[test]
    fn boolean_2() {
        let (rem, parsed) = Boolean::parse(b"false").unwrap();
        assert!(rem.is_empty());
        assert_eq!(parsed, Boolean(false));
    }
    #[test]
    fn boolean_3() {
        let (rem, parsed) = Boolean::parse(b"truebu").unwrap();
        assert_eq!(rem, b"bu");
        assert_eq!(parsed, Boolean(true));
    }
    #[test]
    fn boolean_4() {
        let res = Boolean::parse(b" true");
        assert!(res.is_err())
    }
}
