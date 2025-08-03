// slices are a kind of reference ... a reference to a slice/part of a value
// essentially a pointer that points to only part of a heap value (because collections of values, such as strings, are in the heap)


fn main() {
    // string passed as ref to function, returns index point between first and second word
    let mut my_string = String::from("Goodnight sky!");
    let first_word_index_point:usize = first_word_1(&my_string);
    println!("{first_word_index_point}");
    
    /* commented out because my_string will be used later, and shouldn't be cleared
       the following commented (2-lines) of code shows an issue with not using slices
    my_string.clear();  // var must be mutable to use ... 
    println!("{first_word_index_point}"); // still returns 9 even though the string is empty ... this is why slices exist
    */

    // no space, so index point at the end of the word is returned
    let my_word = String::from("Yowsers!");
    let first_word_index_point_2:usize = first_word_1(&my_word);
    println!("{first_word_index_point_2}");


    // slices examples (continuing to use my_string and my_word): 
    // the following creates two slices "goodnight" and "sky"
    // they borrow their sliced-value from string2
    let goodnight = &my_string[..9];
    let sky = &my_string[9..];
    println!("{my_string} - {goodnight} - {sky}");

    let first_word_returned = first_word_2(&my_string);
    println!("{first_word_returned}");

    let first_word_returned_2 = first_word_2(&my_word);
    println!("{first_word_returned_2}");


    // strings vs string literals
    let my_string_2 = String::from("Lorem ipsum something or other");
    let my_string_2b = "Lorem ipsum something or other";
    println!("{} vs {}",{my_string_2},{my_string_2b});

    // strings are of type String
    // string literals are of type &str (slice, immutable, reference)
}





fn first_word_1(s:&String)->usize {
    // can't return just part of a string without slicing it
    // therefore, only the index point of where it would be sliced is returned

    let bytes = s.as_bytes();
    // iter and enumerate will be studied in a later project (should be 13 and 6 respectively)
    for (index, &character) in bytes.iter().enumerate() {
        if character == b' ' {  // test for space char as bytes type
            return index;   // returns the space char
        }
    } 

    // didn't return in the conditional, so the whole string is a word
    return s.len()
}


// returns a borrowed string slice (&str), not a string (&String)
// param s:&str because &String can be passed as a slice and this will also allow slices to be passed into the function
fn first_word_2(s:&str)->&str {
    let bytes = s.as_bytes();
    for (index, &character) in bytes.iter().enumerate() {
        if character == b' ' {
            return &s[..index]  // found a char, slice here
        }
    }

    return &s[..] // entire string as borrowed "slice", because no spaces were found
}
