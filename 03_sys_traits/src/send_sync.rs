

// Send: 如果一个类型 T 实现了 Send trait, 意味着 T 可以安全的从一个线程移动到另一个线程
// T: Send, 则 T 在某个线程中的独占访问是线程安全的

// Sync: 如果一个类型 T 实现了 Sync trait, 意味着 &T 可以安全的在多个变量中共享。
// T: Sync, 则 T 在线程间的只读共享是安全的

// Rc 是既没有实现 Send， 也没有实现 Sync
// RefCell 实现了 Send， 没有实现 Sync

// 多线程情况下，Send/Sync 的 Arc， 和 Mutex 一起 -> 构造一个可以在多线程间共享且可修改的类型

use std::sync::{Arc, Mutex};
use std::thread;

fn arc_mutext_is_send_sync() {
    let a = Arc::new(Mutex::new(1));
    let b = a.clone();
    let c = a.clone();
    let handle = thread::spawn(move || {
        let mut g = c.lock().unwrap();
        *g += 1;
    });

    {
        let mut g = b.lock().unwrap();
        *g += 1;
    }

    handle.join().unwrap();
    println!("a={:?}", a);
}


fn main() {
    arc_mutext_is_send_sync();
}