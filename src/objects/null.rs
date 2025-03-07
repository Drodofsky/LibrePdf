use nom::{IResult, Parser, bytes::complete::tag};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Null;

impl Null {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Null> {
        tag("null").map(|_| Null).parse(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_null_1() {
        let (rem, parsed) = Null::parse(b"null").unwrap();
        assert!(rem.is_empty());
        assert_eq!(parsed, Null);
    }
    #[test]
    fn parse_null_2() {
        let parsed = Null::parse(b"nul");
        assert!(parsed.is_err())
    }
}
