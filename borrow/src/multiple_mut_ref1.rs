#[allow(unused_mut)]

fn main() {
    let mut data = vec![1, 2, 3];

    // 不可变引用
    let data1 = vec![&data[0]];
    println!("data[0]: {:p}", &data[0]);

    // Rust 下， 不能同时拥有可变引用和只读引用
    // for i in 0..100 {
    // 可变引用发生
    //     data.push(i);
    // }

    println!("data[0]: {:p}", &data[0]);

    // 不可变引用
    println!("boxed: {:p}", &data1);
}
