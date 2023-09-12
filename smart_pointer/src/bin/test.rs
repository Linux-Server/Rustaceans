fn main(){

    let x = String::from("sam");
    let b = &x;
    let c = "sam";
    // let d = *b;

    assert_eq!(c, *b);  


    let a = 10;
    let b = &a;
    assert_eq!(10, b);
}