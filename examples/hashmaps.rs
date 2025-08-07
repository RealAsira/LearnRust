// HASH MAPS (dynamic sized type or collection stored on heap)
// type HashMap<K, V> where K is the key type and V is the value type

// not brought into scope automatically, unlike vectors and strings
use std::collections::HashMap;      // use the HaspMap type

// come back to this example when prompted
// enum can be used as the key type in a HashMap
#[derive(Debug, Hash, Eq, PartialEq)] // derive Debug, Hash, Eq, and PartialEq traits for the enum
enum HashKeyTypes {
    Name(String),   // store as a name
    ID(i32),        // store as an ID
}

fn main() {
    let mut scores:HashMap<&str, i32> = HashMap::new();    // create a new HashMap
    scores.insert("Blue", 10);          // insert a key-value pair
    scores.insert("Yellow", 50);
    println!("{:?}\n", scores);

    // borrowed scores to loop through the HashMap
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // .unwrap() returns the Some value but crashes if None
    // .unwrap_or(999) returns the value of 999 if None or the Some value if Some
    // .unwrap_or() also expects an Option<V> so .copied() is used to convert from Option<&V>
    let pink_score = scores.get("Pink").copied().unwrap_or(0); // get the value for the key "Blue"
    println!("Pink got {:?} points.", pink_score);

    // let pink_name = String::from("Pink"); // won't work... why?
    // because previous hash keys used &str instead of String ... cannot mix and match key types or value types
    // &str is faster but must be careful with lifetimes
    // String is slower but can be used with any string value and the value is owned by the HashMap so lifetime doesn't matter

    // using an &str key type
    let pink_score = 25;
    scores.insert("Pink", pink_score);
    scores.insert("Pink", 30); // update the value for the key "Pink"
    println!("Scores: {:?}", scores);

    // check if a key exists, enter if not
    if !scores.contains_key("Green") {
        scores.insert("Green", 20); // insert a new key-value pair
    }

    // another way (shorthand using entry)
    scores.entry("Red").or_insert(15); // insert a new key-value pair if it doesn't exist
    scores.entry("Red").or_insert(20); // key-value pair already exists, so this does nothing
    println!("Scores after adding new: {:?}\n", scores);


    // go back to the HashKeyTypes enum example
    // the following HashMap uses the HashKeyTypes enum as the key type
    let mut scores2:HashMap<HashKeyTypes, i32> = HashMap::new();
    scores2.insert(HashKeyTypes::Name(String::from("White")), 15);
    scores2.insert(HashKeyTypes::ID(2), 15);
    println!("Scores2: {:?}", scores2);

    // get value from key "White"
    let white_score = scores2.get(&HashKeyTypes::Name(String::from("White"))).copied().unwrap_or(0);
    println!("White got {:?} points.", white_score);


    // operating on HashMap values
    // create a new HashMap to count words (where word is key and count is value)
    let mut word_map:HashMap<&str, i32> = HashMap::new();
    
    // create then loop over some text, word by word
    let some_text = String::from("There once was a was a once");
    for word in some_text.split_whitespace() { // split the string into words
        // count is occuances of each word, or 0 by default
        let count = word_map.entry(word).or_insert(0);
        *count += 1; // increment the count for the word
    }
    println!("Word map: {:?}", word_map);
}
