use std::collections::HashMap;

fn main() {
    let mut v: Vec<u32> = vec![1,2,3];
    v.push(4);
    v.push(5);
    for i in &mut  v {
        *i *= 10;
        println!("{}", i)
    }

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    for i in row {
        println!("{:?}", i);
    }

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    // let s3 = String::new() + &s1 + &s2; // this should create an empty string and then copy s1 and s2 to it and return it
    println!("{}", s3);
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("{} - {}", s, hello);
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores.insert(String::from("Blue"), 25); // Udpate Blue
    scores.entry(String::from("Yellow")).or_insert(50); // Only insert if doesn't exist
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).unwrap();
    println!("{} - {}", team_name, score);
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
