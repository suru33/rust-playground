struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        username,
        email,
        sign_in_count: 1,
        active: true,
    }
}

fn main() {
    let user1 = User {
        username: String::from("SuRu"),
        email: String::from("suru@mail.com"),
        sign_in_count: 10,
        active: true,
    };

    println!("User 1 name: {}", user1.username);

    let user2 = build_user(String::from("email@gmail.com"), String::from("Mr Email"));
    println!("user2 count: {}", user2.sign_in_count);

    let user3 = User {
        username: String::from("User 3"),
        email: String::from("u3@gmail.com"),
        ..user2
    }; // equal to
    // let user3 = User {
    //     email: String::from("another@example.com"),
    //     username: String::from("anotherusername567"),
    //     active: user2.active,
    //     sign_in_count: user2.sign_in_count,
    // };

    println!("User3 email: {}", user3.email);
    println!("User3 active: {}", user3.active);

    // structs without named fields

    struct Point2D(i32, i32);

    let p1 = Point2D(-89, 24);
    println!("p1.X = {}", p1.0);
    println!("p1.Y = {}", p1.1);
}
