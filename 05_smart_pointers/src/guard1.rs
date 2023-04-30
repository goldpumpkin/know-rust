use std::sync::Mutex;

fn main() {
    let mutex = Mutex::new(Mutex::new(1));
    let guard = mutex.lock().unwrap();

    {
        rayon::join(
            || {
                let mut g1 = guard.lock().unwrap();
                *g1 += 1;
                println!("Thread 1: {:?}", *g1);
            },
            || {
                let mut g1 = guard.lock().unwrap();
                *g1 += 1;
                println!("Thread 1: {:?}", *g1);
            }

        )
    };

}