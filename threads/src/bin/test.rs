use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        let num = vec![1, 2, 3, 4,7,99,0,4,6];

        for i in num {
            // println!("sending {i}");
            tx.send(i).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    let mut test1 = Test(44,rx);
   let b= thread::spawn(move||{
        for i in test1.next().unwrap() {
            println!("{:?}",i);
        }

    });
    b.join().unwrap();
    
}

struct Test(i32,Receiver<i32>);

impl Iterator for Test {
    type Item = Vec<i32>;
    fn next(&mut self) -> Option<Self::Item>{
        let mut vec_one=Vec::new();
        
        loop{
            let result= self.1.recv();
            match result{
            Ok(result)=>vec_one.push(result),
            Err(error)=>break
            }
        }
        Some(vec_one) 
    }
    }