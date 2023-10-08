pub fn main() {
  println!();
  println!("references and borrowing!!");

  fn1();

  fn3();

  main_dangle();
}

fn fn1() {
  let s1 = String::from("hello");

  // &s1: s1의 값을 참조하지만 소유하지는 않는 참조자를 생성하도록 해준다
  let len = calculate_length(&s1);

  println!("s1: {}, len: {}", s1, len);
}

fn calculate_length(s: &String) -> usize {
  s.len()
}

fn _fn2() {
  let s = String::from("hello");

  _change(&s);
}

fn _change(_some_string: &String) {
  // 에러: 참조하는 어떤 것을 변경하는 것은 허용되지 않음
  // some_string.push_str(", world"); 
}

fn fn3() {
  let mut s = String::from("hello");

  change(&mut s);
}

fn change(some_string: &mut String) {
  some_string.push_str(", world");
  // println!("some_string: {}", some_string);
}

fn main_dangle() {
  let reference_to_nothing = dangle();

  print!("reference_to_nothing: {}", reference_to_nothing);
}

fn dangle() -> String {
  let s = String::from("hello");

  s
}

