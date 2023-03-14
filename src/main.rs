#[derive(Debug, Clone)]
struct User {
    username: String,
    email: String,
    sign_in_count: i32,
    active: bool,
}


fn main() {
    let user1 = User {
        active: true,
        username: String::from("SampleUser"),
        email: String::from("sample@email.com"),
        sign_in_count: 5,
    };

    let mut user2 = new_user_with(user1.clone(), String::from("sample2@email.com"));
    user2.active = false;
    
    println!("The user: {:#?} {:#?}", user1, user2);
}

fn new_user_with(prime_user: User, email: String) -> User {
    User {
        email,
        ..prime_user
    }
}
