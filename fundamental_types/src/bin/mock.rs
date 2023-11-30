fn main() {
    println!("The mockk");
}

trait Messanger {
    fn send(&self, message: &str);
}

struct LimitTracker<T> {
    messenger: T,
    value: usize,
    max: usize,
}
