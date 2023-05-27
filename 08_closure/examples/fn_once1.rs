fn main() {
    let name = String::from("Tyr");

    let c = move |greeting: String| (greeting, name);

    let result = c("hello".to_string());

    println!("result: {:?}", result);

    // c 已经不能被再次调用了
    // let result = c("hello".to_string());
}