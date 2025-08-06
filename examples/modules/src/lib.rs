// use resturaunt module from ./src/resturaunt/mod.rs
// re-export resturaunt module for use in other modules (pub)
pub mod resturaunt;


pub mod lib_inline {
  pub fn print_lib_inline() {
    println!("Lib inline module!");
  }
}
