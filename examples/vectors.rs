// "collection" is another term for a dynamically sized type (DST)
// e.g. vectors (sizeable heap arrays), strings, and hashmaps

enum ValidNumericsForVector3 {
    Integer(i32),
    Float(f64),
    UInteger(u32),
}


fn main() {
    // vectors
    let mut v2:Vec<i32> = Vec::new();   // create an empty vector of type i32
    let mut v = vec![1, 2, 3];          // create a vector (more standard)

    v.push(4);
    v.push(5);
    v.push(6);

    println!("{:?}", v);    // print the vector

    v.pop();                // remove the last element
    v.pop();
    v.pop();

    println!("{:?}", v);

    println!("{}", v[0]);   // read an element

    // .get() returns an Option<T> enum, which can be None or Some(value)
    let third_element:Option<&i32> = v.get(2);
    match third_element {
        Some(third) => {println!("Third element: {third}") /* add more code here if desired */},
        None => println!("No third element"),
    }
    println!("Second element: {:?}\n\n", v.get(1)); // prints None if index is out of bounds


    // now usign v2 from earlier
    for i in 1..6 {    // 1,2,3,4,5 range (exclusive of 6)
                       // 1..=5 would achieve the same
        v2.push(i);
    }

    for i in &v2 {      // iterate over the vector
        println!("{i}");
    }

    for el in &mut v2.iter_mut() {  // iterate over the vector mutably
        *el *= 2; // double the value
        // the *el instead of just el is required to dereference the value
        // dereferencing it allows us to modify the value in place
    }

    for i in &v2{
        println!("{i}");
    }
    println!("\n");


    // vectors can store only one type, but they can be of an enumerator type
    let mut v3:Vec<ValidNumericsForVector3> = Vec::new();
    v3.push(ValidNumericsForVector3::Integer(1));
    v3.push(ValidNumericsForVector3::Float(1.5));
    v3.push(ValidNumericsForVector3::UInteger(2));

    for i in &v3 {
        match i {
            ValidNumericsForVector3::Integer(val) => println!("Integer: {val}"),
            ValidNumericsForVector3::Float(val) => println!("Float: {val}"),
            ValidNumericsForVector3::UInteger(val) => println!("UInteger: {val}"),
        }
    }

    // another enumerator example
    let mut v4 = vec![ValidNumericsForVector3::Integer(-1), ValidNumericsForVector3::Float(1.5), ValidNumericsForVector3::UInteger(3)];
    for i in &v4 {
        match i {
            ValidNumericsForVector3::Integer(val) => println!("Integer: {val}"),
            ValidNumericsForVector3::Float(val) => println!("Float: {val}"),
            ValidNumericsForVector3::UInteger(val) => println!("UInteger: {val}"),
        }
    }
}
