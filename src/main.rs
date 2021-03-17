#[derive(Debug)]
enum Kind {
    NormalUser,
    TeamLead(u32),
    Manager(String),
}

#[derive(Debug)]
struct User {
    name: String,
    email: String,
    age: u32,
    kind: Kind
}

fn match_test(user: &User) {
    match &user.kind {
        Kind::NormalUser => println!("NormalUser"),
        Kind::TeamLead(years) => println!("Team lead with {} years exp", years),
        Kind::Manager(dept) => println!("Manager of {}", dept),
    }
    if let Kind::TeamLead(years) = &user.kind {
        println!("Extra check {}", years);
    }
}

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
