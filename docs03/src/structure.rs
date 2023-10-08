#![allow(unused)]

pub fn main() {
  println!();
  println!("structure!!");

  #[derive(Debug)]

  struct User {
      username: String,
      email: String,
      sign_in_count: u64,
      active: bool,
  }

  let user1 = User {
      email: String::from("someone@example.com"),
      username: String::from("someusername123"),
      active: true,
      sign_in_count: 1,
  };

  let user2 = User {
      email: String::from("another@example.com"),
      username: String::from("anotherusername567"),

      // active: user1.active,
      // sign_in_count: user1.sign_in_count,
      ..user1
  };

  println!("user1: {:#?}", user1);
  println!("user2: {:#?}", user2);

  fn1();
}

fn fn1() {
  #[derive(Debug)]
  struct Color(i32, i32, i32);
  #[derive(Debug)]
  struct Point(i32, i32, i32);
  
  
  let black = Color(0, 0, 0);
  let origin = Point(0, 0, 0);

  println!("black: {:#?}, origin: {:#?}", black, origin);
}