pub struct Sachin{
    pub name:String
}

impl Summary for Sachin{
    type Item = String;
    fn neat() -> Self::Item {
        "Biila".to_string()
    }
}

pub trait Summary{
    type Item;
    fn neat()->Self::Item;
}