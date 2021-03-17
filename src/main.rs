mod user;
use user::*;

fn main() {
    let u1 = User {
        name: String::from("Trump"),
        email: String::from("trump@usa.com"),
        age: 60,
        kind: Kind::TeamLead(10)
    };
    println!("{:?}", u1);
    let u2 = User {
        name: String::from("Not Trump"),
        kind: Kind::Manager(String::from("Office")),
        email: u1.email.clone(),
        ..u1
    };
    println!("{:?}", u2);
    match_test(&u1);
}
