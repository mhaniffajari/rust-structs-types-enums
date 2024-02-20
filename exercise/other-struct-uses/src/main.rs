struct User {
    username: String,
    email: String,
    url : String,
    isactive: bool,
}

struct Point(i32, i32);

fn main() {
    let username = String::from("hanifajari");
    let email = String::from("muhammadhaniffajari@gmail.com");
    let url = String::from("www.hanifajari.com");
    let isactive = true;

    let user = User {
        username,
        email,
        url,
        isactive,
    };
    let my_point = Point(10, 5);
    println!("Username: {}", user.username);
    println!("Point1 :{}", my_point.0);
}