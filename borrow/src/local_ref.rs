fn main() {
    let r = local_ref();
    println!("r: {:p}", r);
}

#[allow(unused_variables)]
fn local_ref<'a>() -> &'a i32 {
    let a = 42;
    // &a
    todo!()
}