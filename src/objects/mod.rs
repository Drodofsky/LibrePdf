#![cfg_attr(
    test,
    allow(clippy::unwrap_used, clippy::expect_used, clippy::indexing_slicing)
)]

mod array;
mod boolean;
mod dictionary;
mod name;
mod null;
mod number;
mod stream;
mod string;
pub use array::*;
pub use boolean::*;
pub use dictionary::*;
pub use name::*;
use nom::{IResult, Parser, branch::alt};
pub use null::*;
pub use number::*;
pub use stream::*;
pub use string::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Object<'b> {
    Boolean(Boolean),
    Name(Name<'b>),
    Integer(Integer),
    Real(Real),
    String(String),
    Array(Array<'b>),
    Null(Null),
    Dictionary(Dictionary<'b>),
    Stream(Stream<'b>),
}

impl<'b> Object<'b> {
    pub fn parse(input: &'b [u8]) -> IResult<&'b [u8], Object<'b>> {
        alt((
            Name::parse.map(Object::Name),
            Integer::parse.map(Object::Integer),
            Stream::parse.map(Object::Stream),
            Dictionary::parse.map(Object::Dictionary),
            String::parse.map(Object::String),
            Real::parse.map(Object::Real),
            Boolean::parse.map(Object::Boolean),
            Array::parse.map(Object::Array),
            Null::parse.map(Object::Null),
        ))
        .parse(input)
    }
}

pub trait GetObj<T> {
    fn get_obj(&self) -> Option<&T>;
}

macro_rules! impl_get_obj {
    ($obj:ident) => {
        impl GetObj<$obj> for Object<'_> {
            fn get_obj(&self) -> Option<&$obj> {
                if let Object::$obj(o) = self {
                    return Some(o);
                }
                None
            }
        }
    };
}

macro_rules! impl_get_obj_lt {
    ($obj:ident) => {
        impl<'b> GetObj<$obj<'b>> for Object<'b> {
            fn get_obj(&self) -> Option<&$obj<'b>> {
                if let Object::$obj(o) = self {
                    return Some(o);
                }
                None
            }
        }
    };
}
impl_get_obj_lt!(Array);
impl_get_obj_lt!(Dictionary);
impl_get_obj_lt!(Stream);
impl_get_obj!(Boolean);
impl_get_obj_lt!(Name);
impl_get_obj!(Integer);
impl_get_obj!(Real);
impl_get_obj!(String);
impl_get_obj!(Null);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_boolean() {
        let (rem, obj) = Object::parse(b"true").unwrap();
        assert!(rem.is_empty());
        let obj: &Boolean = obj.get_obj().unwrap();
        assert!(obj.get())
    }
    #[test]
    fn parse_name() {
        let (rem, obj) = Object::parse(b"/name").unwrap();
        assert!(rem.is_empty());
        let obj: &Name = obj.get_obj().unwrap();
        assert_eq!(obj.get(), b"name")
    }
    #[test]
    fn parse_integer() {
        let (rem, obj) = Object::parse(b"5").unwrap();
        assert!(rem.is_empty());
        let obj: &Integer = obj.get_obj().unwrap();
        assert_eq!(obj.get(), 5)
    }
    #[test]
    fn parse_real() {
        let (rem, obj) = Object::parse(b"5.").unwrap();
        assert!(rem.is_empty());
        let obj: &Real = obj.get_obj().unwrap();
        assert_eq!(obj.get(), 5.0)
    }
    #[test]
    fn parse_lit_string() {
        let (rem, obj) = Object::parse(b"(str)").unwrap();
        assert!(rem.is_empty());
        let obj: &String = obj.get_obj().unwrap();
        assert_eq!(obj.get(), b"str")
    }
    #[test]
    fn parse_hex_string() {
        let (rem, obj) = Object::parse(b"<abc>").unwrap();
        assert!(rem.is_empty());
        let obj: &String = obj.get_obj().unwrap();
        assert_eq!(obj.get(), [0xab, 0xc0])
    }
    #[test]
    fn parse_array() {
        let (rem, obj) = Object::parse(b"[ 3.14 -5 true (Ralph) /SomeName ]").unwrap();
        assert!(rem.is_empty());
        let obj: &Array = obj.get_obj().unwrap();
        assert_eq!(obj.get().len(), 5)
    }
    #[test]
    fn parse_null() {
        let (rem, obj) = Object::parse(b"null").unwrap();
        assert!(rem.is_empty());
        let obj: &Null = obj.get_obj().unwrap();
        assert_eq!(*obj, Null)
    }
    #[test]
    fn parse_dictionary() {
        let (rem, parsed) = Dictionary::parse(b"<</Name (Prinz)>>").unwrap();
        assert!(rem.is_empty());
        let v: &String = parsed.get(&Name::new(b"Name")).unwrap().get_obj().unwrap();
        assert_eq!(v.get(), b"Prinz");
    }
    #[test]
    fn stream_1() {
        let (rem, parsed) = Object::parse(b"<</Length 6>>stream\nstream\nendstream").unwrap();
        assert!(rem.is_empty());
        let stream: &Stream = parsed.get_obj().unwrap();
        let length: &Integer = stream
            .get_info()
            .get(&Name::new(b"Length"))
            .unwrap()
            .get_obj()
            .unwrap();
        assert_eq!(length.get(), 6);
        let data = stream.get_data();
        assert_eq!(data, b"stream");
    }
}
