use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();
    scores.insert("blue".to_owned(), 10);
    scores.insert("yellow".to_owned(), 50);
    println!("scores = {:?}", scores);

    let teams = vec!["blue".to_owned(), "yellow".to_owned()];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("scores = {:?}", scores);

    let teams = vec!["blue".to_owned(), "yellow".to_owned()];
    let initial_scores = vec![10, 50];
    let mut scores = teams
        .into_iter()
        .zip(initial_scores.into_iter())
        .collect::<HashMap<_, _>>();
    println!("scores = {:?}", scores);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    #[allow(clippy::for_kv_map)]
    for (_, value) in &mut scores {
        *value *= 2;
    }
    println!("scores = {:?}", scores);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    println!("scores = {:?}", scores);
    scores.insert(String::from("Blue"), 20);
    println!("scores = {:?}", scores);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("scores = {:?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
