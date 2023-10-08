#![allow(unused)]

pub fn main() {
  println!();
  println!("if_let!!!");

  fn1();
  fn2();
}

fn fn1() {
  // let some_u8_value = Some(0u8);
  // match some_u8_value {
  //   Some(123) => println!("three"),
  //   _ => (),
  // }

  let some_u8_value = Some(0u8);

  println!("some_u8_value: {:#?}", some_u8_value);


  if let Some(3) = some_u8_value {
    println!("three");
  }
}

fn fn2() {
  #[derive(Debug)]

  enum UsState {
    Alabama,
    Alaska,
  }

  enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
  }

  let coin = Coin::Penny;
  let mut count = 0;
  match coin {
      Coin::Quarter(state) => println!("State quarter from {:?}!", state),
      _ => count += 1,
  }

  println!("count: {}", count);
}
