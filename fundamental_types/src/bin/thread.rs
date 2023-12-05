use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    let t1 = thread::spawn(move || {
        let v1 = vec!["sam", "ram", "jam", "sem"];
        for i in v1 {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    let t2 = thread::spawn(move || {
        let v1 = vec!["sam", "ram", "jam", "sem"];
        for i in v1 {
            tx1.send(i).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    for r1 in rx {
        println!("Got: {}", r1);
    }
}

fn threads2() {
    let (tx, rx) = mpsc::channel();
    let t1 = thread::spawn(move || {
        println!("Im t1");
        tx.send("Hello Rx, Im Tx")
            .expect("Unable to send data to thread");
    });
    thread::sleep(Duration::from_millis(1));
    let r1 = rx.try_recv().expect("Didnt recive the data from t1");
    println!("The data {:?}", r1);
}

fn threads() {
    let x = vec![1, 2, 3, 4, 5];

    let h1 = thread::spawn(move || {
        println!("The vec is {:?}", x);
    });

    // println!("The x val is {:?}", x);

    h1.join().unwrap();
}

fn test1() {
    println!("Start..");

    let h1 = thread::spawn(|| {
        for i in 0..5 {
            println!("Sub threat activated");
            thread::sleep(Duration::from_millis(1));
        }
    });

    h1.join().expect("Unable to run the h1 thread");

    for i in 0..5 {
        println!("Main thread");
        thread::sleep(Duration::from_millis(1));
    }
}
