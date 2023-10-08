pub fn main() {
  println!();
  println!("ownership_2!!");

  let s1 = String::from("hello");

  let (s1, len) = calculate_length(s1);

  println!("s1: {}, len: {}", s1, len);
}

fn calculate_length(s: String) -> (String, usize) {
  let length = s.len();

  (s, length)
}