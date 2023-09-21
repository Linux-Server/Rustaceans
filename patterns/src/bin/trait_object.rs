fn main(){
    let inner_data = Box::new(Button{width:10, height:20, label: "login".to_string()});
    let inner_data1 = Box::new(TextField{color:"red".to_string(), height:44, name: "Usename".to_string()});
    let my_vector:Vec<Box<dyn Draw>>=vec![inner_data, inner_data1];
    let data = Screen{ components: my_vector };

}

trait Draw{
    fn draw(&self);
}
struct Screen{
    components: Vec<Box<dyn Draw>>
}

struct Button{
    width: i32,
    height:i32,
    label: String
}

impl Draw for Button{
    fn draw(&self) {
        println!("Im drawing button")
    }
}

struct TextField{
    color: String,
    height:i32,
    name: String
}

impl Draw for TextField{
    fn draw(&self) {
        println!("Im drawing button")
    }
}
