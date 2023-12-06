use std::sync::Mutex;

fn main() {
    println!("the mutex with shared state concurrency");

    let m = Mutex::new(10);

    let mut x = m.lock().unwrap();
    *x = 33;

    println!("the mutex with shared state concurrency {:?}", m);
}
