// if it has a value, add one, otherwise do nothing
// Input Option<T> (of i32 type here), return the same
fn add_one(x: Option<i32>) -> Option<i32> {
    // use match to test if Option::None or Option::Some<T> enum
    // ie None or Some as Rust uses these keywords to represent Option::None and Option::Some
    match x {
        None => None,           // return Option::None (no type)
        Some(i) => Some(i+1),   // return Option::Some<i32> since i var is i32 and rust assigns it
                                // note: Some(i) uses "i" where i binds to the input variable (x)
        // MATCH RULE: match MUST cover all possible values, compiler error if not
        // use the following as a "catch-all":
        // other => [SOME RESULT],  // returns a value
        // another catch-all
        // _ => [SOME CODE],        // doesn't return a value, but does execute additional code
    }
}





fn main() {
    let five = Some(5);                 // assign an option value of "Some(T)" where T is type
    let six = add_one(five);            // increase by one
    let empty = None;                   // assign an option value of "None"
    let empty_added = add_one(empty);   // add_one (not really) to empty

    println!("five: {:?}  six: {:?}  empty: {:?}  empty_added: {:?}", five, six, empty, empty_added);

    if empty == None {
        println!("IS NONE");
    }

    if empty.is_none() {
        println!("IS NONE");
    }

    if five != None {
        println!("IS SOME");
    }

    if five.is_some() {
        println!("IS SOME");
    }

    if five == Some(5) {
        println!("IS SOME");
    }
}
