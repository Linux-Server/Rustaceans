fn main(){
    // clousre is anonymous func that can save on a varibale
    //pass as args to another func
    //can create in one place and call somehwre else
    //unlike function , closure can captute the varibles in the scope they have defined in

    println!("The closure test");
    let collection = Inventory{shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Red]};
    let choice =None;  
    let outs = collection.give_away(choice);
    println!("the outs : {:?}", outs);
}

#[derive(Debug)]
enum ShirtColor{
    Red,
    Blue
}

struct Inventory{
    shirts: Vec<ShirtColor>
}

impl Inventory{
    fn give_away(&self, shirt_choosen: Option<ShirtColor>)-> ShirtColor{
        let res = shirt_choosen.unwrap_or_else(|| self.call_inventory());
        res

    }

    fn call_inventory(&self)-> ShirtColor{
        let mut blue= 0;
        let mut red = 0;

        for sh in &self.shirts{
            println!("The shirts consist.. {:?}", sh);
            match sh{
                ShirtColor::Blue => blue += 1,
                ShirtColor::Red => red +=1, 
            }

        }
        if blue>red{
           ShirtColor::Blue
        }else{
           ShirtColor::Red
        }
    }
}