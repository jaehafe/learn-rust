#![allow(unused)]

pub fn main() {
  println!();
  print!("program using structure");

  fn1();
  fn2();
}

fn fn1() {
  let length1 = 50;
  let width1 = 30;

  println!(
      "square pixels: {}",
      area(length1, width1)
  );

  let rect1 = (50, 30);

  println!(
      "square pixels rectangle: {}",
      area1(rect1)
  );
}

fn area(length: u32, width: u32) -> u32 {
  length * width
}

fn area1(dimensions: (u32, u32)) -> u32 {
  dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
  length: u32,
  width: u32,
}

fn fn2() {
  let rect1 = Rectangle { length: 50, width: 30 };

  println!("rect1: {:?}", rect1);
}