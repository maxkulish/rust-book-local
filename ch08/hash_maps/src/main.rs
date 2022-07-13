use std::collections::HashMap;

fn main() {

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 20);

    scores.insert(String::from("Yellow"), 25);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    println!("Scores: {:?}", scores);

    let team_name = String::from("Green");
    let score = scores.get(&team_name);

    match score {
        Some(num) => println!("score: {}", num),
        None => println!("There are no such team"),
    }

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Red")).or_insert(100);
    println!("{:?}", scores);

    let txt = "hello world wonderful world strange world";

    let mut map = HashMap::new();

    for word in txt.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
