// "collection" is another term for a dynamically sized type (DST)
// e.g. vectors (sizeable arrays), strings, and hashmaps

fn main() {
    // strings are vector of bytes (not actually an array of characters!)
    // a few ways to create a string:
    let mut s = "Hello".to_string();                // create a new string from a string literal
    let mut s2 = String::new();                     // create a new empty string
    let mut s3 = String::from("We meet at last.");   // create a new string from a string literal

    // because strings are mutable and are vectors, they can be changed
    s.push_str(" there");   // append a string to the end of the empty string
    s.push('!');           // append a single character to the end of the string
    println!("{}", s);

    let s2 = s2 + "General Kenobi!?";   // concatenate two or more string slices
    println!("{}", s2);

    let s4 = format!("{}\n{} {}", s, s2, s3);   // format a string with multiple arguments (much like println!)
    println!("{}", s4);

    /*
    iterating over a string is complicated in Rust
    because strings are not arrays of characters, but rather a vector of bytes.
    therefore, what does the dev want to do? iterate over the bytes, characters, or graphemes?
    */

    // iterate over characters
    for char in s.chars() {
        println!("{}", char);
    }

    // iterate over bytes
    for byte in s.bytes() {
        println!("{}", byte);
    }
}
