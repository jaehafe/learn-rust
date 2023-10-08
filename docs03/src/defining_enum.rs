#![allow(unused)]
pub fn main() {
  fn1();
  fn3();
}

fn fn1() {
  #[derive(Debug)]
  enum IpAddrKind {
    V4,
    V6,
  }

  let four = IpAddrKind::V4;
  let six = IpAddrKind::V6;

  println!("four: {:#?}", four);
  println!("six: {:#?}", six);

  fn route(ip_type: IpAddrKind) { 
    
  }

  route(IpAddrKind::V4);
  route(IpAddrKind::V6);
}

fn fn2() {
  enum IpAddrKind {
    V4,
    V6,
  }

  struct IpAddr {
      kind: IpAddrKind,
      address: String,
  }

  let home = IpAddr {
      kind: IpAddrKind::V4,
      address: String::from("127.0.0.1"),
  };

  let loopback = IpAddr {
      kind: IpAddrKind::V6,
      address: String::from("::1"),
  };
}

enum Fruit {
  Apple(String),
}

fn fn3() {
  let a = Fruit::Apple(String::from("Apple"));

  match a {
    Fruit::Apple(value) => println!("apple: {}", value),
    _ => {},
  }

  // if let Fruit::Apple(value) = a {
  //   println!("{}", value);
  // }
}