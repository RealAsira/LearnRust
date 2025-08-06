pub fn order_breakfast(toast_kind: &str) {
  let mut meal = super::kitchen::Breakfast::summer(toast_kind); // order wheat toast for breakfast
  // meal.toast = String::from("French"); // modify order
  println!("Ordered {}", meal.parts()); // print the parts of the meal
  super::kitchen::cook_order(); // call the public function from Kitchen
}
