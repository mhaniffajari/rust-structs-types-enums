#[derive(Debug)]

struct Person {
    first_name : String,
    last_name : String,
    age : u8
}

fn main() {
    let hanif = Person{first_name : "Muhammad".to_string(),
    last_name: "Hanif".to_string(),
    age : 23};
    // println!("{:?}",Person{first_name : "John".to_string()
    // , last_name : "Doe".to_string(),age:18}
    //println!("{:?}",hanif);
    println!("First Name : {}",hanif.first_name);
    println!("Last Name : {}",hanif.last_name);
    println!("Age : {}",hanif.age);
}