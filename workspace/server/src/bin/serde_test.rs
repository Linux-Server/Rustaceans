use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty as samray};
fn main() {
    println!("Serde create");
    let dog = Dog {
        name: "Jason".to_string(),
        year: 1994,
        owner: DogOwner {
            first_name: "Sam".to_string(),
            last_name: "ray".to_string(),
        },
    };
    let dog_ser = samray(&dog);
    if let Ok(json_data) = dog_ser {
        println!("The json data is : {}", json_data);
    }

    let raw_str = r#" {
  "name": "Jason",
  "year": 1994,
  "owner": {
    "first_name": "Sam",
    "last_name": "ray"
  }
}"#;

    let de_ser = from_str::<Dog>(raw_str);
    if let Ok(val) = de_ser{
        println!("The Dog is : {:?}", val);
    }
}

#[derive(Serialize, Deserialize, Debug)]
// #[serde(rename_all = "camelCase")]
struct Dog {
    name: String,
    year: i32,
    owner: DogOwner,
}

#[derive(Serialize, Deserialize, Debug)]
// #[serde(rename_all = "camelCase")]
struct DogOwner {
    first_name: String,
    last_name: String,
}
