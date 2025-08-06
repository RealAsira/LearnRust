// breakfast and its implementation are public, but some parts are private
pub struct Breakfast {
  pub toast: String,    // public field
  fruit: String,        // private field
}


impl Breakfast {
  pub fn summer(toast: &str) -> Breakfast {
    Breakfast {
      toast: String::from(toast),
      fruit: String::from("peach"), // default fruit
    }
  }

  pub fn parts(&self) -> String {
    format!("Toast: {}, Fruit: {}", self.toast, self.fruit)
  }
}


pub fn cook_order() {
  println!("Order cooked!");
  super::deliver_order();
}


// only accessible within the kitchen module
fn recook_order() {
  println!("Order recooked!");
}


// public function that uses private function and parent function
pub fn fix_order() {
  recook_order();
  super::deliver_order();   // call the function from the parent module
}