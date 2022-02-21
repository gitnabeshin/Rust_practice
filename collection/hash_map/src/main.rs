use std::collections::HashMap;

fn use_hashmap() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);

    let key = String::from("KEY");
    let value = String::from("VALUE");
    let mut map = HashMap::new();
    map.insert(key, value);
    println!("{:?}", map);
    // this is not allowed: key, value is moved when insert
    // println!("{},{}", key, value);

    let key = String::from("KEY");
    let value = String::from("VALUE");
    let mut map = HashMap::new();
    map.insert(&key, &value);
    println!("{:?}", map);
    println!("{},{}", key, value);

}

fn use_hashmap2() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 100);
    scores.insert(String::from("Yellow"), 500);
    println!("scores[{}]: {:?}", scores.len(), scores);
    for (key, val) in &scores{
        println!("   {}: {}", key, val);
    }

    let name = String::from("Blue");
    let val = scores.get(&name);
    println!("Get: key={}, value={:?}", name, val);
}

fn over_write(){
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 10);
    println!("scores[{}]: {:?}", scores.len(), scores);
    for (key, val) in &scores{
        println!("   {}: {}", key, val);
    }

    scores.insert(String::from("Blue"), 20);
    println!("scores[{}]: {:?}", scores.len(), scores);
    for (key, val) in &scores{
        println!("   {}: {}", key, val);
    }

    // entry().or_insert() will insert only when same any key exists.
    scores.entry(String::from("Green")).or_insert(30);
    scores.entry(String::from("Blue")).or_insert(30);
    println!("scores[{}]: {:?}", scores.len(), scores);
    for (key, val) in &scores{
        println!("   {}: {}", key, val);
    }
}

fn count_entry(text: &str) {
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        // or_insert() returns a mutable reference to the value in the entry.
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("map[{}]: {:?}", map.len(), map);
    for (key, val) in &map{
        println!("   {}: {}", key, val);
    }
}

fn main() {
    use_hashmap();
    use_hashmap2();
    over_write();
    count_entry("Hello world nice world");
}
