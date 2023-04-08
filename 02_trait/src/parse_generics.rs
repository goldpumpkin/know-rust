use std::f64;
use std::str::FromStr;

use regex::Regex;

pub trait Parse {
    fn parse(s: &str) -> Self;
}

impl<T> Parse for T
    where
        T: FromStr + Default
{
    fn parse(s: &str) -> Self {
        let re: Regex = Regex::new(r"^[0-9]+(\.[0-9]+)?").unwrap();
        let d = || Default::default();
        if let Some(captures) = re.captures(s) {
            captures.get(0)
                .map_or(d(), |s| s.as_str().parse().unwrap_or_else(|_| d()))
        } else {
            d()
        }
    }
}

#[test]
fn parse_should_work() {
    assert_eq!(u32::parse("123abc"), 123);
    assert_eq!(u32::parse("123.00abc"), 0);
    assert_eq!(f64::parse("123.00abc"), 123.00);
    assert_eq!(f64::parse("123.00abc").to_string(), "123");
}

fn main() { println!("result: {}", u8::parse("255 test parse")) }