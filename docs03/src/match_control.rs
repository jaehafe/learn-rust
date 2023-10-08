#![allow(unused)]

pub fn main() {
  println!();
  println!("match_control!!");

  let value = value_in_cents(Coin::Quarter(UsState::Alabama));
  println!("coin value: {}", value);

  fn1();
}

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

fn value_in_cents(coin: Coin) -> u32 {
  match coin {
      Coin::Penny => {
          println!("Lucky penny!");
          1
      },
      Coin::Nickel => 5,
      Coin::Dime => 10,
      Coin::Quarter(state) => {
        println!("State quarter from {:?}!", state);
        25
    },
  }
}

/////////////////////////////////////
fn fn1() {
  fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
  }

  let five = Some(5);
  let six = plus_one(five);
  let none = plus_one(None);

  print!("five: {:#?}, six: {:#?}, none: {:#?}", five, six, none);
}

fn fn2() {
  let some_u8_value = 0u8;
  match some_u8_value {
      1 => println!("one"),
      3 => println!("three"),
      5 => println!("five"),
      7 => println!("seven"),
      _ => (),
  }
}

enum Fruit {
  Apple(String, u8),
}

fn fn4() {
  let a = Fruit::Apple(String::from("Apple"), 1);

  match a {
    Fruit::Apple(str, num) => println!("{} {}", str, num),
    _ => {},
  }

  // if let Fruit::Apple(value, num) = a {
  //   println!("{} {}", value, num);
  // }
}