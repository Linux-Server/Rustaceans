fn main(){
    // PATTERN = EXPRESSION
    let x = 10;

    let (a,b,c) = (1,2,3);
    //    let (a,b) = (1,2,3); this wont work

    //DESTRUCTURE AVOIDING

    let (x, _) = (1,2);  
    let (x, ..) = (1,2,3,4,5);

}