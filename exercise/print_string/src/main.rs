fn print_str(s:&str) {
    let new_string  = format!("{}{}", s, "!");
    println!("{}",new_string);
}

fn print_string(mut s : String){
    println!("{}",s);
}

fn main() {
    let s = "Hello, World";
    print_str(s);
    print_string(s.to_string());
    let mut salutation = String ::from("Hello");
    print_string(salutation);
}
