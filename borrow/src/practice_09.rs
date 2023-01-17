use std::sync::Arc;

fn main() {
    let arr = Arc::new(vec![1]);
    std::thread::spawn(move || { println!("{:?}", arr); });
}