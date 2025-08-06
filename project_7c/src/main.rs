// look for modules and children in lib.rs
// NOTE: these "uses" only apply to the current module
use project_7c::Kitchen as Back;    // aliasing Kitchen "as" the "Back" of the Diner
pub use project_7c::Diner; // can be used in parent modeules (though that is not really the case here because this is main.rs... this idea, however, is called re-exporting)
// use crate::* to import everything from the root (project_7c) crate

mod resturant {
    pub fn place_order() {
        //Diner::order_breakfast("French"); // WON'T WORK
        super::Diner::order_breakfast("French"); // target parent module
    }
}

fn main() {
    Diner::order_breakfast("Wheat"); // place breakfast order
    println!("The order was not satisfactory.");
    Back::fix_order(); println!("\nNew customer:");

    resturant::place_order(); // place order from restaurant module
}
