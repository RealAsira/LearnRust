// this is the resturaunt module
// could also be stored as ./src/resturaunt.rs with submodules in ./src/resturaunt/

// re-export kitchen and diner modules for use in other modules (pub)
pub mod kitchen;
pub mod diner;

// private function, not accessible outside this module
fn deliver_order() {
  println!("Order delivered!");
}
