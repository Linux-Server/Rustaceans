fn main(){
    let inner_data = Box::new(10);
    let inner_data1 = Box::new("hello".to_string());
     let my_vector:Vec<Box<dyn Draw>> =vec![inner_data, inner_data1];

     let data = Screen{ components: my_vector };

}

trait Draw{
    fn draw(&self);
}
struct Screen{
    components: Vec<Box<dyn Draw>>
}

impl Draw for i32{
    fn draw(&self) {
        
    }
}
impl Draw for String{
    fn draw(&self) {
        
    }
}