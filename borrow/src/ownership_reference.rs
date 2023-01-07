/**
 * borrow : & - 创建某个变量的引用
 * 可变引用： &mut  - 创建某个变量可变引用（即允许修改）
 **/
fn main() {
    let data = vec![1, 2, 3, 4];
    let data1 = &data;
    let data2 = &data;
    let data3 = &data1;
    let data4 = &&&data;

    // data 本身是不可变引用 则会报错
    //let data3 = &mut data;

    // 一样
    // 原因：& 对某个所有者进行借用 data1 和 data2 的值是一样的
    println!(
        "addr of value:  data1:{:p}, data2:{:p}, &&data:{:p}, data3:{:p}",
        data1, data2, &&data, data3
    );

    // 不一样
    // 原因：打印的是 引用在栈上的地址
    println!(
        "addr of stack:  &&data1:{:p}, &&data2:{:p}",
        &&data1, &&data2
    );

    // data1 data2 一样, data3 data4 不一样
    // 原因：打印的是指针指向的内存地址
    println!(
        "addr of Pointer Point:  &*data1:{:p}, &*data2:{:p}, &*data3:{:p}, &*data4:{:p}",
        &*data1, &*data2, &*data3, &*data4
    );

    // 地址：值 vs 引用
    println!(
        "addr of value: {:p}({:p}), addr of data {:p}, data1: {:p}",
        &data, data1, &*data, &data1
    );
    println!("sum of data1: {}", sum(data1));

    println!(
        "addr of items: [{:p}, {:p}, {:p}, {:p}]",
        &data[0], &data[1], &data[2], &data[3]
    )
}

fn sum(data: &[u32]) -> u32 {
    // 地址是否会改变：值 vs 引用
    println!("addr of value: {:p}, addr of ref: {:p}", data, &data);
    data.iter().sum()
}

