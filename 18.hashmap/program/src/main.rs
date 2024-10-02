use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("scores: {:?}", scores);

    let teams = vec![String::from("Red"), String::from("Black")];
    let initial_scores = vec![120, 100];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("scores: {:?}", scores);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    println!("map: {:?}", map);
    // println!("field_name: {}, field_value: {}", field_name, field_value);
    println!("map.get(&String::from(\"Favorite color\"): {:?}", map.get(&String::from("Favorite color")) );
    // println!("map.get(&String::from(\"Favorite color\"): {:?}", map.get(String::from("Favorite color")) );

    println!("");
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    println!("");
    scores.insert(String::from("Yellow"), 100);
    println!("scores: {:?}", scores);

    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Orange")).or_insert(100);
    println!("scores: {:?}", scores);

    let text = "hello world wonderful world";
    let mut h = HashMap::new();

    for word in text.split_whitespace() {
        let count = h.entry(word).or_insert(0);
        *count += 1;
    }
    println!("h: {:?}", h);
}