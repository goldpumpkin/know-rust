use std::cell::RefCell;

fn main() {
    let data = RefCell::new(1);
    {
        // 获得 RefCell 的可变借用
        let mut ref_mut = data.borrow_mut();
        *ref_mut += 1;
    }
    print!("data is {:?}", data);
}
