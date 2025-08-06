fn deliver_order() {
  println!("Order delivered!");
}



pub mod Kitchen {
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
}



pub mod Diner {
  pub fn order_breakfast(toast_kind: &str) {
    let mut meal = super::Kitchen::Breakfast::summer(toast_kind); // order wheat toast for breakfast
    // meal.toast = String::from("French"); // modify order
    println!("Ordered {}", meal.parts()); // print the parts of the meal
    super::Kitchen::cook_order(); // call the public function from Kitchen
  }
}