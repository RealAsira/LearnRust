
fn main() {
    let mut x = 5;  // mut allows the variable to be mutable
    println!("x1: {x}");
    x = 10;         // updating the value
    println!("x2: {x}");

    // overwriting-shadowing a variable
    let x = 15;
    println!("xb: {x}");

    let spaces = "   ";
    let spaces = spaces.len();  // variable was never mutable but it was overwritten
    // this is especially useful since variable types can't be changed ...
    // e.g setting a mut variable = "three" then = 3 would fail as the type can't change
    // overwriting it with let would allow the change

    println!("_: {spaces}");

    const Y:i32 = 42069;    // consts are ALWAYS immutable and MUST have a type-annotation
    println!("i32: {Y}");

    // rust is statically-typed, but is flexible in that types are inferred when possible
    // scalar types are single value types
    // signed integers i8, i16, i32 (default!), i64, i128
    // unsigned ints   u8, u16, u32, u64, u128
    // isize and usize (signed and unsigned ints of the same architecture size as the CPU ... eg 64-bit, 32-bit, etc)
    // f32 and f64 - all floats are signed, f64 is default
    // bools are true or false
    // " represents a string (e.g. let my_string:String = "Goodnight!";)
    // ' represents a single character (e.g let my_char:c = 'Z';)

    // compound types hold more than one variable at a time
    // tuples such as let my_tupple:(i32, f64, u8) = (500, 6.4, 1);
    // access by let val = my_tupple.0; ... my_tupple.1 ... my_tupple.2, etc
    // once sized, tuples length is constant

    let mut my_tupple:(i32, f64, u8) = (500, 6.4, 1);
    my_tupple.0 = -500;
    println!("Tupple: {}", {my_tupple.0});
    
    // arrays such as let my_arr:[i32; 5] = [1,2,3,4,5];
    // access by let val = my_arr[0], my_arr[1], etc
    // let my_arr2 = [3;5]; // creates array [3,3,3,3,3]

    // change array value
    let mut my_arr:[i32; 5] = [1,2,3,4,5];
    my_arr[0] = 6;
    println!("Array: {}", {my_arr[0]});

    // rust's version of tern is inline if else
    let z:i32 = if true {20} else {40};
    println!("{z}");
 
    // functions can be nested
    // additionally, 
    fn test() {
        println!("hello");
    }
    test();

    my_func(Y);

    // calling result of func
    println!("{}", double(x));
    println!("{}", half(x));

    // three loop types
    // continue always skips to next iteration of a loop
    // break always exits a loop
    // loops until "break;"
    loop {
        println!("Needs break condition");
        break;
    }

    // loops while condition is true
    let mut num = 3;
    while num != 0 {
        println!("{num}");
        num -= 1;
    }
    
    // iterate once for each item in an array
    let numbers = [10,20,30,40,50];
    for number in numbers {
        println!("{number}");
    }
    // use .rev() to go reverse order
}


// all function parameters MUST have a type annotation
// they can be declared anywhere in the code then called later
// e.g. my_func is delcared below, but called in the main function which was earlier
fn my_func(x:i32) {
    println!("x is {x}");
}

// functions with return values must have a return type ->TYPE
fn double(x:i32)->i32 {
    return x*2  // no semi-colon ... semi-colon is the end of a statement, not expression
}

fn half(x:i32)->i32 {
    return(x/2);    // semi-colon is okay here, but the compiler warns that the parens are unnecessary
}
