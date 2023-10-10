use std::thread;

fn main(){
  let vec1 = vec![1,2,3,4,5,6];

  let x = |x:i32| vec1;

  let sam = x(10);

  println!("{:?}", sam);
  
}

// fn -> take refernec
// fnmut -> take mut refrence
//fnonce -> takes ownership