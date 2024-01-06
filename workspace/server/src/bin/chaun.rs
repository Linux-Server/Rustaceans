fn main() {
    println!("Chain in iterator");

    let b1 = vec!["sam".to_string(),"ram".to_string()];
    let b2 = vec!["jam".to_string(),"kill".to_string()];



    let a1 = vec![1,2,3,4,5];
    let a2 = vec![10,12,13];

    let c = b1.iter().chain(b2.iter());

    // for i in c{
    //     println!("The c is {:?}", i);
    //
    // }

    println!("The a1 ids {:?} and {:?}",  b1, b2);

    println!("The a1 ids {:?} ",  c);

}