use std::io;

fn main() {
  let mut my_string = "".to_string(); // empty string

  io::stdin()
    .read_line(&mut my_string)
    .expect("Should have read user-input for 'my_string'");

  let my_string = my_string.trim();

  println!("Gave: {}", {my_string});
}
