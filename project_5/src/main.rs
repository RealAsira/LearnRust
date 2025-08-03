// a struct is a multi-key data type, defined by the developer

// struct definition
struct User {
    active:bool,
    username:String,
    email:String,
    sign_in_count:u64
}

// tuple-struct defintion
struct Color(u8,u8,u8);



fn main() {
    // new instance of struct (whole instance is mutable)
    let mut user1:User = User {
        active: true,
        username: String::from("TaliaShaffer"),
        email: String::from("talia@example.com"),
        sign_in_count: 0
    };

    user1.sign_in_count = 1;    // change a value
    println!("{} {} {} {}", {&user1.active},{&user1.username},{&user1.email},{&user1.sign_in_count});   // call values
    
    
    // create another user, copy some data from user1
    let user2:User = User {
        active:false,
        ..user1
    };

    println!("{} {} {} {}", {&user2.active},{&user2.username},{&user2.email},{&user2.sign_in_count});   // call values


    // tuple-structs
    // similar to structs but allow a reusable structure with a specific name
    let black:Color = Color(0,0,0);
    let r:u8 = black.0;
    let g:u8 = black.1;
    let b:u8 = black.2;
    println!("{},{},{}",r,g,b);
}
