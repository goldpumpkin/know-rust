use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

#[derive(Debug)]
enum Language {
    Rust,
    TypeScript,
    Elixir,
    Haskell,
}

impl AsRef<str> for Language {
    fn as_ref(&self) -> &str {
        match self {
            Language::Rust => "Rust",
            Language::TypeScript => "TypeScript",
            Language::Elixir => "Elixir",
            Language::Haskell => "Haskell"
        }
    }
}

fn print(v: impl Into<IpAddr>) {
    println!("{:?}", v.into());
}

fn print_ref(v: impl AsRef<str>) {
    println!("{}", v.as_ref());
}

// 从引用到引用的转换
fn main() {
    let v4: Ipv4Addr = "2.2.2.2".parse().unwrap();
    let v6: Ipv6Addr = "::1".parse().unwrap();

    print(v4);
    print(v6);

    // IPAddr 实现了 From<[u8; 4]，转换 IPv4 地址
    print([1, 1, 1, 1]);
    // IPAddr 实现了 From<[u16; 8]，转换 IPv6 地址
    print([0xfe80, 0, 0, 0, 0xaede, 0x48ff, 0xfe00, 0x1122]);

    let lang = Language::Rust;
    print_ref(lang);

    // &str 实现了 AsRef<str>
    print_ref("Hello Rust");
    // String 实现了 AsRef<str>
    print_ref("Hello Rust".to_string());
}