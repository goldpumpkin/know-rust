use std::str::FromStr;

use regex::Regex;

fn main() {
    let str = "id";
    let result = Regex::from_str(str);
    match result {
        Ok(r) => {println!("regex:{}", r)},
        Err(e) => {println!("not regex:{}", e)},
    };
}