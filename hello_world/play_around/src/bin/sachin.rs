fn main(){
    println!("Hello Sachin");
}


#[cfg(test)]

mod test{
    #[test]
    fn tester(){
        assert!(true);
    }
}