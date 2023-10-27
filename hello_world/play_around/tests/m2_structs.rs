#[derive(Debug)]
struct User {
    email: String,
    username: String,
    count: u64,
    active: bool,
}

impl User{
    fn increment_count(&mut self){
        self.count = 100;
    }
}

fn change_name(user: &mut User, new_name: &str){
    user.email = new_name.to_string();
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn struct_test() {
        dbg!("The strct test is active");
        let mut user = User {
            email: "Sachin".to_string(),
            username: "Sachin6624".to_string(),
            count: 1,
            active: true,
        };
        user.username =String::from("Killer");
         change_name(&mut user, "leo");
         user.increment_count();
        dbg!(user);

    }
}
