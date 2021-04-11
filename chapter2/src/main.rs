
mod foo;
mod pkg;

#[derive(Debug)]
struct User {

    username: String,
    email: String,
    sign_in_count: u32,
    active: bool,
}

fn builder_user(username: String, email: String) -> User {
    User {
        username: username,
        email: email,
        sign_in_count: 0,
        active: false,
    }
}

fn main() {
    println!("Hello, world!");
    let user1 = User {
        username: String::from("jaehoon"),
        email: String::from("rival0605@gmail.com"),
        sign_in_count: 0,
        active: true,
    };
    println!("user struct is {:#?}", user1);

    let rect = (50, 30);
    let area = get_area(rect);
    println!("rect is {:#?}", area);

    foo::public_foo();
    foo::bar::public_bar();

    pkg::rect::connect();

}

fn get_area(rect: (u32, u32)) -> u32 {
    rect.0 * rect.1
}

