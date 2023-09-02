fn main(){

    let x =20;
    let res;
    {
        let y = 10; 
        res = longest(&x);
    }

    println!("{:?}", res); //reference is must not outlive value of lifetime

}


fn longest(x:&i32)->&i32{
    x  // smallest lifetime of y
}