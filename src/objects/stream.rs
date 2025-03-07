use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{multispace0, multispace1},
    sequence::{delimited, preceded, terminated},
};

use super::{Dictionary, GetObj, Integer, Name};

#[derive(Debug, Clone, PartialEq)]
pub struct Stream<'b> {
    pub info: Dictionary<'b>,
    pub data: &'b [u8],
}

impl<'b> Stream<'b> {
    pub fn parse(input: &'b [u8]) -> IResult<&'b [u8], Stream<'b>> {
        let (rem, stream) = terminated(
            Dictionary::parse,
            delimited(multispace0, tag("stream"), multispace1),
        )
        .parse(input)
        .map(|(rem, info)| {
            let length: &Integer = info.get(&Name::new(b"Length"))?.get_obj()?;
            let length: usize = length.get().try_into().ok()?;
            let data = rem.get(0..(length))?;
            let rem = rem.get(length..)?;

            Some((rem, Stream { info, data }))
        })?
        .ok_or(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::TakeWhileMN,
        )))?;

        preceded(multispace0, tag("endstream"))
            .map(|_| stream.clone())
            .parse(rem)
    }

    pub fn get_info(&self) -> &Dictionary<'b> {
        &self.info
    }
    pub fn get_data(&self) -> &'b [u8] {
        self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stream_1() {
        let (rem, parsed) = Stream::parse(b"<</Length 3>>stream\nabc\nendstream").unwrap();
        assert!(rem.is_empty());
        let length: &Integer = parsed
            .get_info()
            .get(&Name::new(b"Length"))
            .unwrap()
            .get_obj()
            .unwrap();
        assert_eq!(length.get(), 3);
        let data = parsed.get_data();
        assert_eq!(data, b"abc");
    }
    #[test]
    fn stream_2() {
        let data: [u8; 93] = [
            0x3C, 0x3C, 0x0A, 0x2F, 0x4C, 0x65, 0x6E, 0x67, 0x74, 0x68, 0x20, 0x35, 0x39, 0x0A,
            0x3E, 0x3E, 0x0A, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6D, 0x0A, 0x78, 0x9C, 0x25, 0xC8,
            0xDB, 0x09, 0xC0, 0x20, 0x14, 0x04, 0xD1, 0x59, 0xF3, 0xD0, 0x10, 0x09, 0xF6, 0x9F,
            0x6E, 0xAC, 0xC3, 0x56, 0x44, 0x59, 0xEE, 0xCF, 0x81, 0x19, 0x60, 0xAD, 0xC4, 0x07,
            0x46, 0x26, 0x99, 0x43, 0xB4, 0x78, 0xA7, 0xB9, 0xC4, 0x8C, 0xBC, 0x4D, 0xD6, 0xFB,
            0x47, 0x16, 0xF3, 0xA8, 0x0E, 0x50, 0xEB, 0xB0, 0x01, 0xFC, 0x17, 0x06, 0x3E, 0x0A,
            0x65, 0x6E, 0x64, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6D,
        ];

        let stream_data: [u8; 59] = [
            0x78, 0x9C, 0x25, 0xC8, 0xDB, 0x09, 0xC0, 0x20, 0x14, 0x04, 0xD1, 0x59, 0xF3, 0xD0,
            0x10, 0x09, 0xF6, 0x9F, 0x6E, 0xAC, 0xC3, 0x56, 0x44, 0x59, 0xEE, 0xCF, 0x81, 0x19,
            0x60, 0xAD, 0xC4, 0x07, 0x46, 0x26, 0x99, 0x43, 0xB4, 0x78, 0xA7, 0xB9, 0xC4, 0x8C,
            0xBC, 0x4D, 0xD6, 0xFB, 0x47, 0x16, 0xF3, 0xA8, 0x0E, 0x50, 0xEB, 0xB0, 0x01, 0xFC,
            0x17, 0x06, 0x3E,
        ];
        let (rem, parsed) = Stream::parse(&data).unwrap();
        assert!(rem.is_empty());
        let length: &Integer = parsed
            .get_info()
            .get(&Name::new(b"Length"))
            .unwrap()
            .get_obj()
            .unwrap();
        assert_eq!(length.get(), 59);
        let parsed_data = parsed.get_data();
        assert_eq!(parsed_data, stream_data);
    }
}
