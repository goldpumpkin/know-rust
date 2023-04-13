#![allow(unused)]

fn id<T>(x: T) -> T {
    x
}

fn main() {
    let int = id(32);
    let str = id("hello");
    print!("int: {}, str: {}", int, str)
}
