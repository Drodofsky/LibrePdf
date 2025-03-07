use nom::{
    IResult, Parser,
    character::complete::{char, multispace0},
    multi::many0,
    sequence::{delimited, preceded},
};

use super::Object;
#[derive(Debug, Clone, PartialEq)]
pub struct Array<'b>(Vec<Object<'b>>);

impl<'b> Array<'b> {
    pub fn parse(input: &'b [u8]) -> IResult<&'b [u8], Array<'b>> {
        delimited(
            char('['),
            many0(preceded(multispace0, Object::parse)),
            preceded(multispace0, char(']')),
        )
        .map(Array)
        .parse(input)
    }
    pub fn get(&self) -> &[Object<'b>] {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::objects::{Boolean, GetObj, Name, Real};

    use super::*;
    #[test]
    fn parse_array_1() {
        let (rem, parsed) = Array::parse(b"[]").unwrap();
        assert!(rem.is_empty());
        assert!(parsed.get().is_empty())
    }
    #[test]
    fn parse_array_2() {
        let (rem, parsed) = Array::parse(b"[1.0]").unwrap();
        assert!(rem.is_empty());
        let e1: &Real = parsed.get()[0].get_obj().unwrap();
        assert_eq!(e1.get(), 1.0)
    }
    #[test]
    fn parse_array_3() {
        let (rem, parsed) = Array::parse(b"[/size 1.0]").unwrap();
        assert!(rem.is_empty());
        let e2: &Name = parsed.get()[0].get_obj().unwrap();
        assert_eq!(e2.get(), b"size");
        let e2: &Real = parsed.get()[1].get_obj().unwrap();
        assert_eq!(e2.get(), 1.0)
    }
    #[test]
    fn parse_array_4() {
        let (rem, parsed) = Array::parse(b"[ /size 1.0 ]").unwrap();
        assert!(rem.is_empty());
        let e2: &Name = parsed.get()[0].get_obj().unwrap();
        assert_eq!(e2.get(), b"size");
        let e2: &Real = parsed.get()[1].get_obj().unwrap();
        assert_eq!(e2.get(), 1.0)
    }
    #[test]
    fn parse_array_5() {
        let (rem, parsed) = Array::parse(b"[ /size 1.0 []]").unwrap();
        assert!(rem.is_empty());
        let e2: &Name = parsed.get()[0].get_obj().unwrap();
        assert_eq!(e2.get(), b"size");
        let e2: &Real = parsed.get()[1].get_obj().unwrap();
        assert_eq!(e2.get(), 1.0);
        let e3: &Array = parsed.get()[2].get_obj().unwrap();
        assert!(e3.get().is_empty())
    }
    #[test]
    fn parse_array_6() {
        let (rem, parsed) = Array::parse(b"[ /size 1.0 [true]]").unwrap();
        assert!(rem.is_empty());
        let e2: &Name = parsed.get()[0].get_obj().unwrap();
        assert_eq!(e2.get(), b"size");
        let e2: &Real = parsed.get()[1].get_obj().unwrap();
        assert_eq!(e2.get(), 1.0);
        let e3: &Array = parsed.get()[2].get_obj().unwrap();
        let e3: &Boolean = e3.get()[0].get_obj().unwrap();
        assert!(e3.get());
    }
}
