use std::{
    sync::mpsc::{self, Receiver},
    thread,
    time::Duration,
};

fn main() {
    let (tx, rx) = mpsc::channel();

    println!("Main thread start");

    let a = thread::spawn(move || {
        for i in 1..6 {
            thread::sleep(Duration::from_millis(1000));
            tx.send(1).unwrap();
            println!("The sub thread a");
        }
    });

    let mut custom_struct = Test(10, rx);

    //  println!("{:?}", custom_struct.next());
    for i in custom_struct {
        println!("The Custom receiver is getting data....{:?}", i)
    }
}

struct Test(i32, Receiver<i32>);

impl Iterator for Test {
    // we will be counting with usize
    type Item = i32;

    // next() is the only required method
    fn next(&mut self) -> Option<Self::Item> {
        println!("NEXT...");
        // Increment our count. This is why we started at zero.
        self.0 += 1;

        let data = self.1.recv();

        match data {
            Ok(val) => Some(self.0),
            Err(_) => None,
        }

        // println!("The data is {:?}", data);

        // Check to see if we've finished counting or not.
        // if self.0 < 6 {
        //     Some(self.0)
        // } else {
        //     None
        // }
    }
}
