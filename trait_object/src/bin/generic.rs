fn main() {
    call_me("hello"); //This will not work
    call_me1("hello"); // This will work

}

fn call_me<T>(input: &T){
    
}

fn call_me1(input: &str){
    
}
