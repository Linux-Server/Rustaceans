use ShirtColor::{Red,Green,Blue};

fn main(){
    println!("Its an alternative...");
    let inventory = Inventory{shirts:vec![Red,Green,Blue,Red]};
    let user_choice = Some(Green);
    let result = inventory.give_away(user_choice);
    println!("The result is {:?}", result);


}


#[derive(Debug)]
enum ShirtColor{
    Red,
    Green,
    Blue
}

struct Inventory{
    shirts: Vec<ShirtColor>
}

impl Inventory{
    fn give_away(&self, user_pref: Option<ShirtColor>)-> ShirtColor{
        user_pref.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self)-> ShirtColor{
        let mut red_count = 0;
        let mut blue_count = 0;
        let mut green_count = 0;

        for shirt in &self.shirts{
            match shirt{
                Red => red_count += 1,
                Blue => blue_count += 1,
                Green => green_count += 1

            };
        }

        if red_count< blue_count{
            Red
        }else{
            Blue
        }
    }
}