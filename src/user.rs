#[derive(Debug)]
pub enum Kind {
    NormalUser,
    TeamLead(u32),
    Manager(String),
}

#[derive(Debug)]
pub struct User {
    pub name: String,
    pub email: String,
    pub age: u32,
    pub kind: Kind
}

pub fn match_test(user: &User) {
    match &user.kind {
        Kind::NormalUser => println!("NormalUser"),
        Kind::TeamLead(years) => println!("Team lead with {} years exp", years),
        Kind::Manager(dept) => println!("Manager of {}", dept),
    }
    if let Kind::TeamLead(years) = &user.kind {
        println!("Extra check {}", years);
    }
}