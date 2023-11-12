/// HEllo Ist sachin

fn main(){
    let x = vec![0,1,2,3,4,5,6];
    let target = 10;
    let mut tmp = x[0];
    for (index, val) in x.iter().enumerate(){
        if index > 0{
            let check = tmp+ val;
            if check == target{
                break;
            }
        }
        

    }
}

//Plan of Action:
/*
    - create at tmp var and assign first value/ tmp = x[0] (outside loop)
    -  add tmp + x[1]



*/