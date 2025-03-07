use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::take_while,
    character::complete::{char, digit0, digit1},
    combinator::{map, opt, recognize, verify},
    sequence::pair,
};
#[derive(Debug, Clone, Copy, PartialEq)]

pub struct Integer(i32);
#[derive(Debug, Clone, Copy, PartialEq)]

pub struct Real(f32);

impl Integer {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Integer> {
        let sign = opt(alt((char('+'), char('-'))));
        let parse_number = verify(
            take_while(|c: u8| c.is_ascii_digit() || c == b'.'),
            |ca: &[u8]| !ca.contains(&b'.'),
        )
        .map_res(core::str::from_utf8)
        .map_res(str::parse);
        pair(sign, parse_number)
            .map_opt(|(sign, value)| sign_integer(sign, value))
            .parse(input)
    }
    pub fn get(&self) -> i32 {
        self.0
    }
    pub fn new(i: i32) -> Self {
        Self(i)
    }
}

impl Real {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Real> {
        let sign = opt(alt((char('+'), char('-'))));
        let parse_float = recognize(alt((
            map((digit1, opt((char('.'), digit0))), |_| ()),
            map((char('.'), digit1), |_| ()),
        )))
        .map_res(core::str::from_utf8)
        .map_res(str::parse);
        pair(sign, parse_float)
            .parse(input)
            .map(|(rem, (s, v))| (rem, sign_real(s, v)))
    }

    pub fn get(&self) -> f32 {
        self.0
    }
    pub fn new(r: f32) -> Self {
        Self(r)
    }
}

fn sign_integer(sign: Option<char>, value: i32) -> Option<Integer> {
    if let Some(s) = sign {
        if s == '-' {
            return Some(Integer(value.checked_neg()?));
        }
    }
    Some(Integer(value))
}

fn sign_real(sign: Option<char>, value: f32) -> Real {
    if let Some(s) = sign {
        if s == '-' {
            return Real(-value);
        }
    }
    Real(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_integer_1() {
        let (rem, parsed) = Integer::parse(b"123").unwrap();
        assert!(rem.is_empty());
        assert_eq!(parsed.0, 123);
    }
    #[test]
    fn parse_integer_2() {
        let (rem, parsed) = Integer::parse(b"+17").unwrap();
        assert!(rem.is_empty());
        assert_eq!(parsed.0, 17);
    }
    #[test]
    fn parse_integer_3() {
        let (rem, parsed) = Integer::parse(b"-98").unwrap();
        assert!(rem.is_empty());
        assert_eq!(parsed.0, -98);
    }
    #[test]
    fn parse_integer_4() {
        let parsed = Integer::parse(b"1.");
        assert!(parsed.is_err());
    }
    #[test]
    fn parse_integer_5() {
        let parsed = Integer::parse(b".1");
        assert!(parsed.is_err());
    }
    #[test]
    fn parse_integer_6() {
        let parsed = Integer::parse(b" 1");
        assert!(parsed.is_err());
    }
    #[test]
    fn parse_real_1() {
        let (rem, parsed) = Real::parse(b"34.5").unwrap();
        assert!(rem.is_empty());
        assert_eq!(parsed.0, 34.5);
    }
    #[test]
    fn parse_real_2() {
        let (rem, parsed) = Real::parse(b"+34.5").unwrap();
        assert!(rem.is_empty());
        assert_eq!(parsed.0, 34.5);
    }
    #[test]
    fn parse_real_3() {
        let (rem, parsed) = Real::parse(b"-34.5").unwrap();
        assert!(rem.is_empty());
        assert_eq!(parsed.0, -34.5);
    }
    #[test]
    fn parse_real_4() {
        let (rem, parsed) = Real::parse(b"34.").unwrap();
        assert!(rem.is_empty());
        assert_eq!(parsed.0, 34.);
    }
    #[test]
    fn parse_real_5() {
        let (rem, parsed) = Real::parse(b".34").unwrap();
        assert!(rem.is_empty());
        assert_eq!(parsed.0, 0.34);
    }
    #[test]
    fn parse_real_6() {
        let (rem, parsed) = Real::parse(b"34").unwrap();
        assert!(rem.is_empty());
        assert_eq!(parsed.0, 34.0);
    }
    #[test]
    fn parse_real_7() {
        let (rem, parsed) = Real::parse(b"1.2.3").unwrap();
        assert_eq!(rem, b".3");
        assert_eq!(parsed.0, 1.2);
    }
    #[test]
    fn parse_real_8() {
        let parsed = Real::parse(b" 1.2.3");
        assert!(parsed.is_err())
    }
    #[test]
    fn parse_real_9() {
        let (rem, parsed) = Real::parse(b"1.2e-1").unwrap();
        assert_eq!(rem, b"e-1");
        assert_eq!(parsed.0, 1.2);
    }
}
