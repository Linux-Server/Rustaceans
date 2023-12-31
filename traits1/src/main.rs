fn main(){

    let inventory = Shirts{shirt: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue]} ;

    let user1 = Some(ShirtColor::Red);
    // let user2 = None;
    let res = inventory.give_away(user1);
    println!("The res is {:?}", res);

    let x = "sam".to_string();
    let y = ||  x;   //  Fn -> trait


}

struct Shirts{
    shirt: Vec<ShirtColor>
}

#[derive(Debug)]
enum ShirtColor{
    Red,
    Blue
}

impl Shirts{
    fn give_away(&self, user_pref: Option<ShirtColor>)->ShirtColor{
        match user_pref{
            Some(val) => return val,
            None => return self.random_choice()
        }

    }

    fn random_choice(&self)-> ShirtColor{
          let mut red_shirt = 0;
          let mut blue_shirt = 0;

          for color in &self.shirt{
                 match color{
                    ShirtColor::Blue => blue_shirt+=1,
                    ShirtColor::Red => red_shirt+=1
                 }
          }
          if red_shirt > blue_shirt{
             ShirtColor::Red
          }else{
             ShirtColor::Blue
          }
         
    }


}