fn main(){

    let x =20;
    let res;
    {
        let y = 10; 
        res = longest(&x, &y);
    }

    println!("{:?}", res); //reference is must not outlive value of lifetime

}


fn longest<'a>(x:&'a i32, y:&i32)->&'a i32{
    x  // smallest lifetime of y
}