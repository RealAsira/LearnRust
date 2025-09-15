use std::io::ErrorKind;     // for ErrorKind enum
use std::io::{self, Read};  // for reading files
use std::fs;                // for file system operations
use std::fs::File;          // for file operations specifically

fn main() {
    // get a backtrace using `RUST_BACKTRACE=1 cargo run`

    /*
    // panic! macro kills the application
    // then rust cleans up the program (by default)
    // finally, the message is printed

    panic!("This is a panic message from main.rs"); 
    */


    /*
    // a programmatic error causes fatal error (panic)
    let v = vec![1,2,3];
    v[99];  // read beyond the end of the vector
    */


    // RECOVERABLE ERRORS
    /*
    uses built in Result enum:
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
    */

    let file_path = "C:\\Users\\Asira\\Projects\\learn_rust\\project_9\\src\\2.txt";
    let file_result = File::open(file_path);
    
    // handle the Result type returned by File::open
    let res = match file_result {
        Ok(file) => file,                // return file because it was found and opened
        Err(e) => match e.kind() {      // an error occured, handle it
            ErrorKind::NotFound => {
                // file was missing, attempt to create it instead (panic on creation failure)
                match File::create(file_path) {
                    Ok(new_file) => new_file,   // created new file, return it
                    Err(e) => panic!("Error creating file: {}", e),
                }
            },
            _ => panic!("Error opening or creating file: {}", e),    // catch-all for other errors
        }
    };

    println!("File found or created: {:?}", res);

    // shorthand to run .panic!() on error - default error message
    let _file_path_3 = "C:\\Users\\Asira\\Projects\\learn_rust\\project_9\\src\\3.txt";
    // let file_3 = File::open(_file_path_3).unwrap(); // unwrap will panic if the file cannot be opened

    // another shorthand to run .panic!() on error - custom error message
    let _file_path_4 = "C:\\Users\\Asira\\Projects\\learn_rust\\project_9\\src\\4.txt";
    //let file_4 = File::open(_file_path_4)
    //    .expect("4.txt should be included but is missing"); // expect will panic with a custom message if the file cannot be opened


    // bubbling errors (propagating errors returned by a function using Result as return type)
    let file_path_5 = "C:\\Users\\Asira\\Projects\\learn_rust\\project_9\\src\\5.txt";
    let file_5 = read_text_from_file(&file_path_5);
    println!("Result from file 5: {:?}", file_5);

    let file_path_6 = "C:\\Users\\Asira\\Projects\\learn_rust\\project_9\\src\\6.txt";
    let file_6 = read_text_from_file_2(&file_path_6);
    println!("Result from file 6: {:?}", file_6);
}



// long way for error propagation
fn read_text_from_file(file_path:&str) -> Result<String, std::io::Error> {
    // open the file
    let some_file = File::open(file_path);

    // handle the Result returned by File::open
    let mut file_result = match some_file {
        Ok(file) => file,             // return the file
        Err(e) => return Err(e),  // return the error if file cannot be opened
    };

    // handle the Result returned by read_to_string
    let mut text_result = String::new();
    match file_result.read_to_string(&mut text_result) {
        Ok(_) => Ok(text_result),          // return the text if read was successful
        Err(e) => Err(e),           // return the error if read failed
    }

    // the result (a string or error) is returned to the caller
}


// shorter way for error propagation using the ? operator
fn read_text_from_file_2(file_path:&str) -> Result<String, io::Error> {
    let mut some_file = File::open(file_path)?;     // if error, return it

    let mut some_text = String::new();
    some_file.read_to_string(&mut some_text)?;       // if error, return it

    return Ok(some_text); // return the text if everything was successful

    /*
    What is happening here ... The ? operator is shorthand for the match statement.
    It automatically returns the error if the Result is an Err variant.
    If the Result is Ok, it extracts the value and continues execution.
    */
}


// even shorter!
fn _read_text_from_file_3(file_path:&str) -> Result<String, io::Error> {
    // string file open and read into one line with ? operators
    let mut some_text = String::new();
    File::open(file_path)?.read_to_string(&mut some_text)?;
    return Ok(some_text);
}

// built in function for file reading (does it have error handling?)
fn _read_text_from_file_4(file_path:&str) -> Result<String, io::Error> {
    // use std::fs::read_to_string to read the file in one line
    return fs::read_to_string(file_path);
}

// ? ERROR HANDLING USING OPTION<T>
