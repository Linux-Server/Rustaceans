use std::collections::LinkedList;

fn main() {
    let mut list_one = Linked::new();
    list_one.push(1);
    list_one.push(2);
    list_one.push(3);

    let mut x = None;
    let c = x.take().map(|e: i32|{
        33
    });

    println!("{:?}", x);
    println!("{:?}", c);


    println!("The first nod eis {:?}", list_one);

}
#[derive(Debug)]
struct Node{
    data: i32,
    next: Option<Box<Node>>
}

#[derive(Debug)]
struct Linked{
    head: Option<Box<Node>>
}
impl Linked{
    fn new()->Linked{
        Linked{
            head: None
        }
    }

    fn push(&mut self, data:i32){
        let new_node = Node{
            data,
            next: self.head.take()
        };
        self.head = Some(Box::new(new_node));

    }

    fn pop(&mut self){
        self.head.take().map(|x|{
            self.head = x.next;
            x.data
        });
    }
}