use std::thread;
use std::time::Duration;

fn main() {
    println!("Start..");

    let h1 =  thread::spawn(||{
        for i in 0..5{
            println!("Sub threat activated");
            thread::sleep(Duration::from_millis(1));
        }
    });

    h1.join().expect("Unable to run the h1 thread");


    for i in 0..5{
        println!("Main thread");
        thread::sleep(Duration::from_millis(1));
    }






}