// enums define possible types/kinds a value can be
#[derive(Debug)]            // can use derive debug just like structs
#[allow(dead_code)]         // can disable dead_code warn like structs
enum IPType {
    V4(String), V6(String)  // type is optional ... no type = no data
}

// enums can also have implementations like structs
impl IPType {
    // method
    fn read_ip(&self) {
        println!("{:?}", self);
    }

    // associated function
    fn new_v4(ipv4:&str)->IPType {
        return Self::V4(ipv4.to_string())
    }
}



#[derive(Debug)]
#[allow(dead_code)]
enum StatesUS {
    Utah, Oregon, Washington
}

#[derive(Debug)]
#[allow(dead_code)]
enum Coin {
    Nickel, Dime, Quarter(StatesUS), Half, Dollar
}

impl Coin {
    fn value_cents(&self)->u8 {
        match self {
            // multiple lines to executing if match
            Coin::Nickel => {
            println!("Lucky nickel");
            5
            },
            // simpler return value or code execute
            Coin::Dime => 10,
            Coin::Half => 50,
            Coin::Dollar => 100,

            // state takes value in quarter
            Coin::Quarter(state) => {
                println!("Quarter from {:?}", {state});
                25
            },
        }
    }
}





// Option<T> ENUMERATOR EXAMPLE:

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





fn route(address:&IPType) {
    println!("IPType called {:?}", address);
}





fn main() {
    // called like associated-functions from struct-implementations
    let ipv4 = IPType::V4("127.0.0.1".to_string());
    let ipv6 = IPType::V6("::1".to_string());
    let ipv4_2 = IPType::new_v4("127.0.0.2");

    // either are valid as they are children of IPType
    route(&ipv4);
    route(&ipv6);
    ipv4_2.read_ip(); // called method



    // match enums
    let my_coin = Coin::Nickel;
    println!("Value: {:?}", my_coin.value_cents());

    let my_quarter = Coin::Quarter(StatesUS::Oregon);
    println!("Value: {:?}", my_quarter.value_cents());


    // Option enum
    let five = Some(5);                 // assign an option value of "Some(T)" where T is type
    let six = add_one(five);            // increase by one
    let empty = None;                   // assign an option value of "None"
    let empty_added = add_one(empty);   // add_one (not really) to empty

    println!("five: {:?}  six: {:?}  empty: {:?}  empty_added: {:?}", five, six, empty, empty_added);

    // various some and none check statements
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
