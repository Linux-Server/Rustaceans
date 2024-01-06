
fn main() {
    println!("Hello world");
    let mut x = vec![1,2,3,4,5,6];
    let y = ["hello".to_string(),"hai".to_string(), "now".to_string()];
    // let mut b = x.iter();
    // while let Some(i) = b.next(){
    //     println!("The val {:?}", i);
    // }


    let mut out = x.iter();
    // iter next()
    assert_eq!(Some(&1), out.next());
    assert_eq!(Some(&2), out.next());

    // all

    let all = x.iter().all(|&val| val>=1);
    assert_eq!(all, true);

    // any

    let any = x.iter().any(|&val| val>3);
    // assert_eq!(all, true);


    // by ref

    let mut x = vec![1,2,3,4,5,6].into_iter();

    let by_ref = x.by_ref().take(2).collect::<Vec<i32>>();
    println!("{:?}", by_ref);
    let remain = x.collect::<Vec<i32>>();
    println!("the remain {:?}", remain);

    let x = vec![1,2];
    let y = x.type_id();
    println!("the : {:?}", y);


}

// fn test<T>(data:T){
//     let out = data.iter();
//     let x = vec![1,2];
//     let y = x.type_id();
//
//     match data.type_name(){
//
//      }
//
// }