#[allow(dead_code,unused_variables)]

fn main(){
    let res: Result<i32,&str> = Ok(10);
    let err_res:Result<i32,&str>  = Err("Unabkle to get data");

    let x = res.map_err(|x| println!("The value is {:?}", x));

}