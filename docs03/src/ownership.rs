pub fn main() {
  fn1();
  fn2();
}

fn fn1() {
  let mut s = String::from("hello");

  s.push_str(", world!");

  println!("{}", s)
}

fn fn2() {
  let s1 = String::from("hello");
  let s2 = s1;

  println!("fn2: {:#?}", s2);
}