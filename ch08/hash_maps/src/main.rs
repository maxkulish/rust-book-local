use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 20);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Yellow")).or_insert(60);

    println!("{:?}", scores);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = 
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    println!("{:?}", scores);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // We aren't able to use the variable field_name and field_value
    // after they've been moved into the hash map
    println!("{:?}", map);

    let field_name = String::from("Favorite color");
    println!("{:?}", map.get(&field_name));

    let text = "hello world wonderful world hello mother";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
