// stack = last in, first out (newer is used first) ... push and pop ... fixed data size
// heap = chunk of memory for storing data ... pointer to heap is on stack ... variable data size

// rust ownership rules:
// each value has an owner
// only one owner at a time
// owner is out of scope? value is dropped
// scope is what parts of a program an owner is valid in
// values are scoped exclusively to the block {...} they are instantiated within

// Strings
// &str is a static-sized-type slice of a String object (immutable) on the stack
// because &str is immutable, it is stored on the stack and is quick/efficient
// String is a dynamically-sized-type heap object (mutable)
// because String is mutable, it is stored on the heap and is not so quick/efficient

// rust follows RAII (resource allocation is initialization)
// resources aren't allocated until an object is initialized

fn main() {
    let mut s = String::from("Goodnight"); // create a DST heap string "Goodnight"
    let _end:&str = " sky!"; // create an SST stack string "sky!" (this piece of code is not used except as an example ... underscore denotes this)
    s.push_str(" sky!");    // push end var onto end of s (last in, first out)
    println!("{s}");


    // making it more complex now...
    let _x = 5;  // x is assigned to 5
    let _y = _x; // the value of x is "copied" to y

    // length = how long a string is
    // capacity = how long it can be
    let s1 = String::from("Goodnight"); // stores a pointer, length, and capacity ... heap stores each letter
    let s2 = s1;                        // pointer, length, and capacity are copied on the stack, but it points to the same heap object (SHARED)

    // assigning var1 to var2 copies JUST the stack data from var1 to var2
    // for something like static-sized-types (SSTs) like ints, all data is copied
    // for something like dynamically-sized-types (DSTs) like Strings, only data like pointer, len, cap are copied because they are on the stack
    // this prevents the actually, lengthy data (each and every character) from being copied
    // that would be expensive AND memory intensive

    // what happens if s1 is brought out of scope but s2 isn't? or vice-a-versa?
    // in some langauges the heap would be cleaned and that would cause issues for the remaining variable
    // (double free error)
    // rust prevents this by over-writing s1 with s2 ... that is, s1 no longer exists but the pointer, len, cap are preserved in s2 ... and the heap is untouched

    // println!("{s1"); would not work now that s2 overwrites s1
    // in short ... assigning a new variable to an f DST variable more-or-less "renames" the original
    println!("{s2}");



    // how about changing the value of a heap object?
    let mut _s3 = String::from("Hiya");  // new heap object
    _s3 = String::from("Heya");

    // the re-assignment:
    // - drops the original heap allocation
    // - creates a new heap allocation (Heya)
    // - updates the pointer stored on the stack (named s3)

    // "deep copy" (more expensive)
    let _s4 = String::from("Yolo");
    let _s5 = _s4.clone();    // creates a NEW heap allocation entierly separate from the first

    // to re-iterate ... ALL stack values (known size) are copied on the stack.
    // this is much like heap's "clone" but instead uses an implied "copy"
    /* for example, the code used earlier:
        let _x = 5;
        let _y = x; // copies x on the stack
    */

    
    // passing heap values into functions ...
    let s6 = String::from("my-oh-my");
    print_string(s6.clone());
    println!("{s6}");   // couldn't use if .clone() wasn't called, as s6 would be dropped and some_string in print_string() is out of scope

    print_string_2(&s6); // passing a borrowed string (reference) to the function instead of cloning is lighter-weight
    println!("{s6}");   // s6 is still valid because it was borrowed, not moved then de-allocated
    

    // recall how assigning var1 to var2 (both DSTs on heap) drops var1 stack pointer?
    // the same thing happens when passing a var (s6) into a function with param (some_string)
    // the s6 stack pointer is removed and replace with some_string
    // thus, .clone() is used to clone the value into some_string
    // this is why the print_string_2 function works with &s6 ... it borrows the pointer to the heap value then returns ownership


    let s7 = String::from("boy-oh-boy");
    let s8 = bounce_string(s7);

    // what happens with s7 and s8?
    // s7 is passed to bounce_string
    // bounce_string's some_string overwrites the old s7 pointer
    // bounce_string returns some_string
    // s8 overwrites the returned some_string
    // s8 now owns "boy-oh-boy"

    println!("{s8}");

    // etc, etc ... assigning a var to another ALWAYS drops the original pointer and the new pointer points to the original heap
    // ie, the old variable can't be used if it is assigned to another ... and it points to a heap object
    // again, this doesn't apply to stack variables because they are stored entirely on the stack and use the underlying "copy" mechanism

    
    /* BORROWING */

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




// prints the string
fn print_string(some_string:String) {
    println!("{some_string}");
}

// borrows the heap string (DST) and prints it
fn print_string_2(some_string:&String) {
    println!("{some_string}");
}

// returns the string exactly as it is ("bounces")
fn bounce_string(some_string:String)->String {
    return some_string
}


/* BORROWING */

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
