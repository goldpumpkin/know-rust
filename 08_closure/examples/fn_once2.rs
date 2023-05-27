fn main() {
    let name = String::from("Tyr");

    let c = move |greeting: String| (greeting, name.clone());

    // 所以 c1 可以被调用多次
    println!("c1 call once: {:?}", c("giao".into()));
    println!("c1 call twice: {:?}", c("bonjour".into()));

    println!("result: {:?}", call_once("hi".into(), c));

    // 无法再次调用
    // let result = c("hi".to_string());

    // fn 也可以被当成 fn 调用，只要接口一致就可以
    println!("result: {:?}", call_once("hola".into(), not_closure))
}

fn call_once(arg: String, c: impl FnOnce(String) -> (String, String)) -> (String, String) {
    c(arg)
}

fn not_closure(arg: String) -> (String, String) {
    (arg, "Rosie".into())
}

