use std::sync::mpsc;
use std::thread;
use std::rc::Rc;
use std::sync::Arc;

fn main() {

    let (tx, rx) = mpsc::channel();
    let tx = Arc::new(tx);
    // let rx = Arc::new(rx);

   let a =  thread::spawn(|| {
        println!("Sender is transmitting...");
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let b =  thread::spawn(|| {
        println!("Sender is transmitting...");
        let val = String::from("hi");
        tx.send(val).unwrap();
    });


   let z =  thread::spawn(|| {      
       let x =  rx.recv().unwrap();
       println!("The receiver is {:?}", x)
    });


    // a.join().unwrap();

}



  // let q = Rc::new(10);
    // let e = Rc::clone(&q);
    // let z = Rc::clone(&q);
    // println!("The rc is {:?}", Rc::strong_count(&q));
