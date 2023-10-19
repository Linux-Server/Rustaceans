#[allow(dead_code)]
#[derive(Debug)]
enum CarColor{
    Red,
    Blue,
    Green
}

#[derive(Debug)]
enum GivenResult<T,U>{
    Ok(T),
    Err(U)
}

fn create_car_color() -> CarColor{
    let first_car = CarColor::Red;
    first_car
}

fn check_under_five(num:i32)-> GivenResult<i32, &'static str>{
    if num<5{
        GivenResult::Ok(num)
    }else{
        GivenResult::Err("Not expected")
    }
}

fn check_under_five_built_in(num:i32)-> Result<i32, &'static str>{
    if num<5{
        Ok(num)
    }else {
        Err("Its an error")
    }
}

fn find_reminder(num:i32)-> Option<i32>{
    if num%2 == 0{
        None
    }else{
        Some(num%2)
    }
}

#[cfg(test)]

mod test{
   use super::*;
    #[test]
    fn test_car_color(){
        let car_color = create_car_color();
        dbg!(car_color);
    }
    #[test]
    fn test_enums(){
        let under_five = check_under_five(20);
        dbg!(under_five);
        let under_five_built_in = check_under_five_built_in(20).unwrap();
        dbg!(under_five_built_in);
        let reminder = find_reminder(11);
        dbg!(reminder);
    }

}
