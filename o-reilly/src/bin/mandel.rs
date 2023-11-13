fn main(){
    println!("Its a mandel bot");
    square_loop(-10.0);
}

fn square_loop(mut x:f64){
    loop{
        x = x * x;
        println!("The value of  x is {:?}", x);
        // break;
    }
}