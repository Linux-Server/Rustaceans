#[allow(dead_code)]


pub struct Post{
    state: Option<Box<dyn State>>,
    contents: String
}

impl Post{
    pub fn new()->Post{
        Post { state: Some(Box::new(Draft{})), contents: String::new() }
    }
    pub fn add_text(&mut self, data:&str){
        self.contents.push_str(data)
    }
}


pub trait State{
    
}

struct Draft{}

impl State for Draft{}