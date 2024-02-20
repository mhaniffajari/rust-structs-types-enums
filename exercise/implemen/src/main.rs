struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User {
    fn new(username: String, email: String) -> Self {
        User {
            username,
            email,
            sign_in_count: 0,
            active: true,
            }
        }
    fn deactivate(&mut self) {
        self.active = false;
    }
}

fn main() {
    let mut new_user = User::new(
        String::from("hanifajari"),
        String::from("muhammadhaniffajari@gmail.com"),
    );
    println!("Username: {}", new_user.username);
    println!("Email: {}", new_user.email);
    new_user.deactivate();
    println!("Active: {}", new_user.active);
}