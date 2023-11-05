use rand::Rng;

fn main(){
    println!("Welcome to password generator");

}

struct Password{
    length:usize
}

impl Password{
    fn new()-> Password{
        Self::with_capacity(10)
    }

    fn with_capacity(length:usize)-> Self{
        Password { length }
    }
}


impl IntoIterator for Password{
    type Item=String;
    type IntoIter: PasswordIterator<Item = Self::Item>;

    // Required method
    fn into_iter(self) -> Self::IntoIter {
         PasswordIterator{
            length: self.length
         }
    }
}

struct PasswordIterator{
    length:usize
}

impl Iterator for PasswordIterator{
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        let mut result = String::with_capacity(self.length);
        for i in 0..self.length {
            let range = 0..=(b'z' - b'a');
            let val = b'a' + rand::thread_rng().gen_range(range);
             result.push(val  as char);
        }
        Some(result)
        
    }
}