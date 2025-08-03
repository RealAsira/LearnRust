// enums define possible types/kinds a value can be
#[derive(Debug)]            // can use derive debug just like structs
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


/*
THE OPTION ENUM REPLACES NULL and looks like the following

enum Option<T> {
    None, Some(T),
}

... Option is included in the standard library but is common enough it doesn't need to be explicitly called
... Option::None and Option::Some are also so common they are called via None and Some
*/



fn main() {
    // called like associated-functions from struct-implementations
    let ipv4 = IPType::V4("127.0.0.1".to_string());
    let ipv6 = IPType::V6("::1".to_string());
    let ipv4_2 = IPType::new_v4("127.0.0.2");

    // either are valid as they are children of IPType
    route(&ipv4);
    route(&ipv6);
    ipv4_2.read_ip(); // called method


    // OPTION ENUM
    let some_number = Some(5);          // automatically infers type ... no annotation needed
    let none_number:Option<i32> = None; // requires type annotation because None can't infer type
    
    // let sum = some_number + none_number; // This causes compile error, because i8 can't add to Option<i8>
}



fn route(address:&IPType) {
    println!("IPType called {:?}", address);
}
