use nom::{
    IResult, Parser, bytes::complete::take_while1, character::complete::char, sequence::preceded,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Name<'b>(&'b [u8]);

impl<'b> Name<'b> {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Name> {
        preceded(
            char('/'),
            take_while1(|c: u8| !b"\n\r%()<>[]{} ".contains(&c)),
        )
        .map(Name)
        .parse(input)
    }
    pub fn get(&self) -> &'b [u8] {
        self.0
    }
    pub fn new(name: &'b [u8]) -> Self {
        Self(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_name_1() {
        let (rem, parsed) = Name::parse(b"/Name1").unwrap();
        assert!(rem.is_empty());
        assert_eq!(parsed.get(), b"Name1")
    }
    #[test]
    fn parse_name_2() {
        let (rem, parsed) = Name::parse(b"/A;Name_With-various***characters?%").unwrap();
        assert_eq!(rem, b"%");
        assert_eq!(parsed.get(), b"A;Name_With-various***characters?")
    }
    #[test]
    fn parse_name_3() {
        let parsed = Name::parse(b"/%");
        assert!(parsed.is_err());
    }
    #[test]
    fn parse_name_4() {
        let (rem, parsed) = Name::parse(b"/A ").unwrap();
        assert_eq!(rem, b" ");
        assert_eq!(parsed.get(), b"A")
    }
}
