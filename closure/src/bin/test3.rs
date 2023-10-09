use std::thread;

fn main(){
  let vec1 = vec![1,2,3,4,5,6];

  let x = move|| vec1;

  let sam = x();

  println!("{:?}", sam);
  
}

// fn -> take refernec
// fnmut -> take mut refrence
//fnonce -> takes ownership