use std::collections::HashMap;

use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::multispace0,
    multi::many0,
    sequence::{delimited, pair, preceded},
};

use super::{Name, Object};

#[derive(Debug, Clone, PartialEq)]
pub struct Dictionary<'b>(HashMap<Name<'b>, Object<'b>>);

impl<'b> Dictionary<'b> {
    pub fn get(&self, key: &Name<'b>) -> Option<&Object> {
        self.0.get(key)
    }
    pub fn parse(input: &'b [u8]) -> IResult<&'b [u8], Dictionary<'b>> {
        delimited(
            tag("<<"),
            many0(pair(
                preceded(multispace0, Name::parse),
                preceded(multispace0, Object::parse),
            )),
            preceded(multispace0, tag(">>")),
        )
        .map(|v| {
            v.into_iter()
                .filter(|(_k, v)| *v != Object::Null(super::Null))
                .collect::<HashMap<Name, Object>>()
        })
        .map(Dictionary)
        .parse(input)
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use crate::objects::{GetObj, Real};

    use super::*;
    #[test]
    fn parse_dictionary_1() {
        let (rem, parsed) = Dictionary::parse(b"<<>>").unwrap();
        assert!(rem.is_empty());
        assert!(parsed.is_empty());
    }
    #[test]
    fn parse_dictionary_2() {
        let (rem, parsed) = Dictionary::parse(b"<</Version 0.1>>").unwrap();
        assert!(rem.is_empty());
        let v: &Real = parsed
            .get(&Name::new(b"Version"))
            .unwrap()
            .get_obj()
            .unwrap();
        assert_eq!(v.get(), 0.1);
    }
    #[test]
    fn parse_dictionary_3() {
        let (rem, parsed) = Dictionary::parse(b"<< /Version 0.1 >>").unwrap();
        assert!(rem.is_empty());
        let v: &Real = parsed
            .get(&Name::new(b"Version"))
            .unwrap()
            .get_obj()
            .unwrap();
        assert_eq!(v.get(), 0.1);
    }
    #[test]
    fn parse_dictionary_4() {
        let (rem, parsed) = Dictionary::parse(b"<</Type /Dic /Version 0.1>>").unwrap();
        assert!(rem.is_empty());
        let v: &Real = parsed
            .get(&Name::new(b"Version"))
            .unwrap()
            .get_obj()
            .unwrap();
        assert_eq!(v.get(), 0.1);
        let t: &Name = parsed.get(&Name::new(b"Type")).unwrap().get_obj().unwrap();
        assert_eq!(t.get(), b"Dic");
    }
    #[test]
    fn parse_dictionary_5() {
        let (rem, parsed) =
            Dictionary::parse(b"<</Type /Dic /Version 0.1/Sub << /Type /Sub >>>>").unwrap();
        assert!(rem.is_empty());
        let v: &Real = parsed
            .get(&Name::new(b"Version"))
            .unwrap()
            .get_obj()
            .unwrap();
        assert_eq!(v.get(), 0.1);
        let t: &Name = parsed.get(&Name::new(b"Type")).unwrap().get_obj().unwrap();
        assert_eq!(t.get(), b"Dic");
        let sd: &Dictionary = parsed.get(&Name::new(b"Sub")).unwrap().get_obj().unwrap();
        let st: &Name = sd.get(&Name::new(b"Type")).unwrap().get_obj().unwrap();

        assert_eq!(st.get(), b"Sub");
    }
    #[test]
    fn parse_dictionary_6() {
        let (rem, parsed) = Dictionary::parse(b"<</var null>>").unwrap();
        assert!(rem.is_empty());
        assert!(parsed.is_empty());
    }
    #[test]
    fn parse_dictionary_7() {
        let (rem, parsed) = Dictionary::parse(b"<</Type /Dic /Version 0.1 /Sub null>>").unwrap();
        assert!(rem.is_empty());
        let v: &Real = parsed
            .get(&Name::new(b"Version"))
            .unwrap()
            .get_obj()
            .unwrap();
        assert_eq!(v.get(), 0.1);
        let t: &Name = parsed.get(&Name::new(b"Type")).unwrap().get_obj().unwrap();
        assert_eq!(t.get(), b"Dic");
        let sub = parsed.get(&Name::new(b"Sub"));
        assert!(sub.is_none())
    }
}
