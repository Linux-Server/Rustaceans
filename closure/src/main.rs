//CLOSURE
/*
Its an anonymous func that can be saved on varibale and pass as a function argumnet


*/

fn main() {
// closure returning an i32 value
  let mut x: String ="sam".to_string(); // owned value
  let y =20; //
  let my_closure = || {
                                       let y: String = x;
                                        
                                     };

   println!("{:?}", x);

//   let z: String = my_closure();
  


}
