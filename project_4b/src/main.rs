// borrowing 

fn main() {
    let s1 = String::from("Goodnight sky!");
    // has to be &s1 because string_length function expects a borrowed string
    let len = string_length(&s1);   
    println!("\"{s1}\" is {len} chars.");
    
    // note: may be worth it to consider the &String and String types to be different in the context of functions
    // because their "parent type", so to speak, is the same... they can be cast back and forth
    
    // another way to look at it ... string_length's string param pointed to s1's pointer which points to the heap value
    

    // this applies to ANY borrowed ownership
    // the following doesn't end up pointing to a heap value, but rather the value stored with num1
    let num1:i32 = 15;
    let num1_doubled:i32 = double(&num1);
    println!("{num1} {num1_doubled}");


    // borrowed variables are immutable by default...
    // to mutate them, it has to be explicitly allowed

    
    // add mut keyword, modify the func call with &mut (reference to mutable var)
    let mut num2:i32 = 15;
    triple(&mut num2);
    println!("{num2}");

    // however, each mutable var can be borrowed only ONE time
    // vars also can't be borrowed as mutable and immutable simultaneously
    // but any number of immutable are allowed

    // it is okay to borrow a var as mutable, even if it was previously borrowed as immutable
    // conditions for this are that either the previous borrow is out of scope (de-allocated) OR the immutable borrows are never called again
}




// &String means a borrowed String ... ie, the ownership is only temporarily transferred
fn string_length(string:&String)->usize {
    return string.len()
}

fn double(number:&i32)->i32 {
    return number*2
}

// allows mutable variables to pass in
fn triple(number:&mut i32) {
    *number = *number*3; // not sure why de-ref is required here. will investigate further.
}
