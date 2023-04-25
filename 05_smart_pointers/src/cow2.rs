use std::borrow::Cow;
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct User<'input> {
    #[serde(borrow)]
    name: Cow<'input, str>,
    age: u8,
}

fn main() {
    let input = r#"{"name":"Tyr", "age":18}"#;
    let user: User = serde_json::from_str(input).unwrap();

    match user.name {
        Cow::Borrowed(x) => println!("Borrowed {}", x),
        Cow::Owned(x) => println!("owned {}", x),
    }
}