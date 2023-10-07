pub fn main() {
  let mut x = 5;
  println!("x value: {}", x);
  x = 6;
  println!("x value: {}", x);
}

pub fn main1() {
  let x: i32 = 5;
  let x: i32 = x + 1;
  let x: i32 = x * 2;

  println!("value x: {}", x);
}

pub fn main2() {
  let spaces = "        ";
  println!("spaces 1: {}", spaces);
  let spaces = spaces.len();
  println!("spaces 2: {}", spaces);
}