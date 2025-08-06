/*
PACKAGE
Packages are bundles of one or more crates.

Contains a Cargo.toml file.
Packages can have any number of library crates, but only one binary crate.


CRATE
Crates are binary or library crates

Binary crates are compiled and executable. These require an fn main() {}
Typically found in src/main.rs.
Library crates don't have a main function and aren't compiled.
E.g. the rand crate or IP crate in this example package.

Crate root - source file the compiler starts from (ie root module)


MODULES 
.rs files or inline mod mod_name {[module code]}
*/


// external modules
mod ip;     // declare a module at /src/ip.rs

// inline module
mod magic {
    pub fn print_msg() {
        println!("Magic module!");
    }
}

use ip::IP;             // use IP struct from ip module
use magic::print_msg;   // use print_msg function from magic module


fn main() {
    let ipv4 = IP::new_v4("192.168.0.1");
    println!("{:?}", ipv4.read_ip());
    print_msg();
}
