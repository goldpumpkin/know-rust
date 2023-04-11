struct User {
    name: String,
}

fn main() {
    let user = User { name: "junmajinlong".to_string() };
    let u = &user;

    // 报错
    // let name = u.name;

    // let u1 = *(&user);

    let s1 = "ABS".to_string();
    let s2 = "CBD".to_string();

    max(&s1, &s2);
}

fn max<'a>(s1: &'a str, s2: &'a str) -> &'a str { if s1 > s2 { s1 } else { s2 } }
