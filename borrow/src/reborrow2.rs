fn main() {
    let mut b = 3;

    // 可变借用，抢到独占锁
    let c = &mut b;
    println!("{}", c);

    // 所有者抢回来
    println!("{:?}", b);

    // 不可变借用，由于出现过可变借用，则继续抢独占锁
    let c1 = &b;
    println!("{}", c1);

    // 所有者随时可以抢回来
    println!("{:?}", b);
}