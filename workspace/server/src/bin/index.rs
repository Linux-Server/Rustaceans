fn main() {
    let x = vec![1, 2, 3, 4, 5];
    let y = vec![10, 20, 30, 40, 50];

    // Consumable Iterator - a method inside iter trait, which call next() method
    //Iterator adaptor - a mathod, which wont call next, but create a new Iterator

    // let c = x.iter().sum::<i32>();
    // let c = x.iter().chain(y.iter()).sum::<i32>();
    let c = x
        .into_iter()
        .chain(y.into_iter())
        .map(|x| x + 2)
        .take(2).collect::<Vec<i32>>().into_iter().take(1).collect::<Vec<i32>>();


    println!("the all vec {:?}", c);
}
