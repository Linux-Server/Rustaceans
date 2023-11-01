fn main(){
   let x ="hello"; //5
   let y: &str ="worldxxx"; //8
   let z = large(x, y);
   println!("The value of r is {:?}", z);
    

}


fn large(x:&str, y:&str)-> &str{
    if x.len()>y.len(){
        x
    }else{
        y
    }
}