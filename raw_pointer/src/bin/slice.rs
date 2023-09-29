fn main(){
let a = [1, 2, 3, 4, 5];

let b = String::from("hello");

let slice = &a[..];

assert_eq!(slice, &[2, 3]);

}