#[derive(Clone, Debug)]
struct Developer {
    name: String,
    age: u8,
    lang: Language,
}

#[derive(Clone, Debug)]
enum Language {
    Rust,
    Java,
    TypeScript,
    Elixir,
    Haskell,
}

// Clone 是深度拷贝
fn main() {
    let dev = Developer {
        name: "goldpumpkin".to_string(),
        age: 12,
        lang: Language::Rust,
    };

    let dev1 = dev.clone();

    println!("dev:{:?}", dev);
    println!("dev1:{:?}", dev1);
}