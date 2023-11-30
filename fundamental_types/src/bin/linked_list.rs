fn main() {
    println!("The inked list activated");
    let n1 = LinkedList::new();
}

#[derive(Debug)]
struct Node {
    data: i32,
    next: Option<Box<Node>>,
}

struct LinkedList {
    head: Option<Node>,
}

impl LinkedList {
    fn new() -> LinkedList {
        Self { head: None }
    }
}
