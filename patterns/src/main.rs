fn main() {
   let favorite_color:Option<&str> = None;
   let is_tuesday = false;
   let age:Result<u8, _> = "29".parse();

   if let Some(color) = favorite_color{
        println!("The color is red");
   }else if is_tuesday{
      println!("Its a tuesday broo");
   }else if let Ok(data) = age {
       println!("Its parsed into {:?}", data);
   }
   
   
   else{
    println!("No color provided")
   }
}
