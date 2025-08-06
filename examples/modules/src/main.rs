/*
To use modules (mod and use) in Rust, the project MUST be a cargo package.
rustc does not support modules.

PACKAGES
Packages are bundles of one or more crates.

Contains a Cargo.toml file.
Packages can have any number of library crates, but only one binary crate.


CRATES
Crates are binary or library crates

Binary crates are compiled and executable. These require an fn main() {}
Typically found as src/main.rs. May bundle library crates.

Library crates don't have a main function and aren't compiled on their own.
E.g. the rand crate or IP crate in this example package.

Crate root - source file the compiler starts from (ie root module)


MODULES 
.rs files or inline mod mod_name {[module code]}
*/

// import a module from ./lib.rs  OR  ./name.rs  OR  ./name/mod.rs  OR  lib.rs
/*
Example module tree
main calls ip and inline
lib calls diner
diner calls front and kitchen

  main.rs
  __|_________
 |     |      |
ip    lib   inline
     __|__
    |     |
  diner  lib_inline
   __|__
  |     |
front kitchen
*/



mod ip;     // import the ip module from ./ip.rs
use ip::IP; // use the IP enum from the ip module


// create an inline module
mod inline {
  pub fn print_inline() {
    println!("Inline module!");
  }
}
use inline::print_inline;         // use the inline module function

// modules is the name of the package
// then the module name in lib.rs
// then the function name
use modules::lib_inline::print_lib_inline; // use the lib_inline module function



// import resturaunt module (and sub-modules)
use modules::resturaunt;



fn main() {
  let my_v4 = IP::new_v4("192.168.0.1");          // create a new IP address
  println!("IP Address: {:?}", my_v4.read_ip());  // read the IP address
  println!("");
  
  print_inline();     // call the inline module function
  print_lib_inline(); // call the lib_inline module function
  println!("");
  
  resturaunt::diner::order_breakfast("Wheat"); // place breakfast order
  println!("The order was not satisfactory.");
  resturaunt::kitchen::fix_order();
}
