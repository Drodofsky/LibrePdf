use nom::{
    IResult, Input, Parser, branch::alt, bytes::complete::take_until, character::complete::char,
    sequence::delimited,
};
type RString = std::string::String;
#[derive(Debug, PartialEq, Clone)]
pub struct String(Vec<u8>);

impl String {
    pub fn parse(input: &[u8]) -> IResult<&[u8], String> {
        alt((Self::parse_hexadecimal, Self::parse_literal)).parse(input)
    }
    pub fn parse_literal(input: &[u8]) -> IResult<&[u8], String> {
        delimited(char('('), take_until_unbalanced_bracket, char(')'))
            .map_res(remove_esc_seq)
            .parse(input)
    }
    pub fn parse_hexadecimal(input: &[u8]) -> IResult<&[u8], String> {
        delimited(char('<'), take_until(">"), char('>'))
            .map_res(core::str::from_utf8)
            .map(fix_hex_str)
            .map_res(hex::decode)
            .map(Self)
            .parse(input)
    }
    pub fn get(&self) -> &[u8] {
        self.0.as_ref()
    }
}

fn take_until_unbalanced_bracket(input: &[u8]) -> IResult<&[u8], &[u8]> {
    let mut balance: i32 = 0;
    let mut is_escaped = false;
    let mut index = 0;
    for (i, c) in input.iter_indices() {
        if c == b'(' && !is_escaped {
            balance = balance.saturating_add(1);
            is_escaped = false;
        } else if c == b')' && !is_escaped {
            balance = balance.saturating_sub(1);
            is_escaped = false;
        } else if c == b'\\' {
            is_escaped = !is_escaped;
        } else {
            is_escaped = false;
        }
        if balance == -1 {
            index = i;
            break;
        }
    }
    let error = nom::Err::Error(nom::error::Error::new(
        input,
        nom::error::ErrorKind::TakeWhile1,
    ));
    if balance != -1 {
        return Err(error);
    }
    Ok((
        input.get(index..).ok_or(error.clone())?,
        input.get(0..index).ok_or(error)?,
    ))
}

fn remove_esc_seq(input: &[u8]) -> Result<String, nom::error::ErrorKind> {
    let mut res = input.to_vec();
    for (i, c) in input.iter_indices().rev() {
        if c == b'\\' {
            if let Some(esc) = input.get(i.saturating_add(1)) {
                match esc {
                    b'\n' | b'\r' => {
                        res.remove(i);
                        while let Some(nl) = res.get(i) {
                            if *nl == b'\n' || *nl == b'\r' {
                                res.remove(i);
                            } else {
                                break;
                            }
                        }
                    }
                    b'n' => {
                        res.remove(i);
                        res.remove(i);
                        res.insert(i, b'\n');
                    }
                    b'r' => {
                        res.remove(i);
                        res.remove(i);
                        res.insert(i, b'\r');
                    }
                    b't' => {
                        res.remove(i);
                        res.remove(i);
                        res.insert(i, b'\t');
                    }
                    b'b' => {
                        res.remove(i);
                        res.remove(i);
                        res.insert(i, 0x08);
                    }
                    b'f' => {
                        res.remove(i);
                        res.remove(i);
                        res.insert(i, 0x0C);
                    }
                    b'(' => {
                        res.remove(i);
                        res.remove(i);
                        res.insert(i, b'(');
                    }
                    b')' => {
                        res.remove(i);
                        res.remove(i);
                        res.insert(i, b')');
                    }
                    b'\\' => {
                        res.remove(i);
                    }

                    o1 @ b'0'..b'8' => {
                        let o2 = is_ascii_digit(input.get(i.saturating_add(2)));
                        let o3 = is_ascii_digit(input.get(i.saturating_add(3)));
                        let mut b: u8 = 0;
                        if o2.0 {
                            b = 1;
                            if o3.0 {
                                b = 2
                            }
                        }
                        let mut n = parse_octal(b, *o1)?;
                        if o2.0 {
                            n = n
                                .checked_add(parse_octal(b.saturating_sub(1), o2.1)?)
                                .ok_or(nom::error::ErrorKind::Digit)?;
                            if o3.0 {
                                n = n
                                    .checked_add(parse_octal(0, o3.1)?)
                                    .ok_or(nom::error::ErrorKind::Digit)?;
                            }
                        }
                        res.remove(i);
                        for _ in 0..=b {
                            res.remove(i);
                        }
                        res.insert(i, n)
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(String(res))
}

fn fix_hex_str(s: &str) -> RString {
    let mut s: RString = s.chars().filter(|c| c.is_ascii_hexdigit()).collect();
    if s.len() % 2 == 1 {
        s.push('0');
    }

    s
}

fn is_ascii_digit(input: Option<&u8>) -> (bool, u8) {
    if let Some(d) = input {
        return (d.is_ascii_digit(), *d);
    }
    (false, 0)
}

fn parse_octal(exp: u8, ascii_number: u8) -> Result<u8, nom::error::ErrorKind> {
    if ascii_number.is_ascii_digit() {
        let number = ascii_number.saturating_sub(b'0');
        if number < 8 {
            return number
                .checked_mul(
                    8u8.checked_pow(exp.into())
                        .ok_or(nom::error::ErrorKind::Digit)?,
                )
                .ok_or(nom::error::ErrorKind::Digit);
        }
    }
    Err(nom::error::ErrorKind::Digit)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_string_1() {
        let (rem, parsed) = String::parse(b"<901fa3>").unwrap();
        assert!(rem.is_empty());
        assert_eq!(parsed.get(), [0x90, 0x1f, 0xa3]);
    }
    #[test]
    fn parse_string_2() {
        let (rem, parsed) = String::parse(b"<901fa>").unwrap();
        assert!(rem.is_empty());
        assert_eq!(parsed.get(), [0x90, 0x1f, 0xa0]);
    }
    #[test]
    fn parse_string_3() {
        let (rem, parsed) = String::parse(b"< 9 0 1 f a 3 >").unwrap();
        assert!(rem.is_empty());
        assert_eq!(parsed.get(), [0x90, 0x1f, 0xa3]);
    }
    #[test]
    fn parse_string_4() {
        let (rem, parsed) = String::parse(b"<  >").unwrap();
        assert!(rem.is_empty());
        assert_eq!(parsed.get(), []);
    }
    #[test]
    fn parse_string_5() {
        let (rem, parsed) = String::parse(b"( This is string number 1? )").unwrap();
        assert!(rem.is_empty());
        assert_eq!(parsed.get(), b" This is string number 1? ");
    }
    #[test]
    fn parse_string_6() {
        let (rem, parsed) = String::parse(b"(strangeonium spectroscopy)").unwrap();
        assert!(rem.is_empty());
        assert_eq!(parsed.get(), b"strangeonium spectroscopy");
    }
    #[test]
    fn parse_string_7() {
        let (rem, parsed) =
            String::parse(b"(This string is split \\\nacross \\\n\\\rthree lines)").unwrap();
        assert!(rem.is_empty());
        assert_eq!(parsed.get(), b"This string is split across three lines");
    }
    #[test]
    fn parse_string_8() {
        let (rem, parsed) = String::parse(b"(string with \\245two octal characters\\307)").unwrap();
        assert!(rem.is_empty());
        let mut esp = b"string with ".to_vec();
        esp.push(0o245);
        esp.append(&mut b"two octal characters".to_vec());
        esp.push(0o307);
        assert_eq!(parsed.get(), esp.as_slice());
    }
    #[test]
    fn parse_string_9() {
        let (rem, parsed) = String::parse(b"(\\24)").unwrap();
        assert!(rem.is_empty());
        assert_eq!(parsed.get(), [0o24]);
    }
    #[test]
    fn parse_string_10() {
        let (rem, parsed) = String::parse(b"(\\24r)").unwrap();
        assert!(rem.is_empty());
        assert_eq!(parsed.get(), [0o24, b'r']);
    }
    #[test]
    fn parse_string_11() {
        let (rem, parsed) = String::parse(b"(\\2)").unwrap();
        assert!(rem.is_empty());
        assert_eq!(parsed.get(), [0o2]);
    }
    #[test]
    fn parse_string_12() {
        let (rem, parsed) = String::parse(b"(\\2r)").unwrap();
        assert!(rem.is_empty());
        assert_eq!(parsed.get(), [0o2, b'r']);
    }

    #[test]
    fn parse_string_13() {
        let (rem, parsed) = String::parse(b"(())").unwrap();
        assert!(rem.is_empty());
        assert_eq!(parsed.get(), b"()");
    }
    #[test]
    fn parse_string_14() {
        let parsed = String::parse(b"(()");
        assert!(parsed.is_err());
    }
    #[test]
    fn parse_string_15() {
        let (rem, parsed) = String::parse(b"(()(()))").unwrap();
        assert!(rem.is_empty());
        assert_eq!(parsed.get(), b"()(())");
    }
    #[test]
    fn parse_string_16() {
        let (rem, parsed) = String::parse(b"(()(()))").unwrap();
        assert!(rem.is_empty());
        assert_eq!(parsed.get(), b"()(())");
    }
    #[test]
    fn parse_string_17() {
        let (rem, parsed) = String::parse(b"(\\))").unwrap();
        assert!(rem.is_empty());
        assert_eq!(parsed.get(), b")");
    }
    #[test]
    fn parse_string_18() {
        let (rem, parsed) = String::parse(b"(\\()").unwrap();
        assert!(rem.is_empty());
        assert_eq!(parsed.get(), b"(");
    }
}

#[cfg(kani)]
#[kani::proof]
fn parse_octal_input() {
    let exp: u8 = kani::any();
    let ascii_number: u8 = kani::any();
    let n = parse_octal(exp, ascii_number);
    if exp >= 3 || ascii_number < b'0' || ascii_number > b'7' {
        assert!(n.is_err())
    } else if exp == 2 && ascii_number >= b'4' {
        assert!(n.is_err())
    } else {
        assert!(n.is_ok())
    }
}
#[cfg(kani)]
#[kani::proof]
fn parse_octal_output() {
    let exp: u8 = kani::any();
    let number: u8 = kani::any();
    kani::assume(exp < 2);
    kani::assume(number <= 7);
    let s = format!("{number}");
    let ascii_number: char = s.chars().next().unwrap();
    let n = parse_octal(exp, ascii_number as u8).unwrap();
    assert_eq!(n, number * (8u8.pow(exp.into())))
}
