pub fn main1() {
  boolean();
  char();
  tuple();
}

pub fn boolean() {
  let _t = true;

  let _f: bool = false; // with explicit type annotation
}

pub fn char() {
  let c = 'z';
  let z = 'ℤ';
  let heart_eyed_cat = '😻';

  println!("char: {} {} {}", c, z, heart_eyed_cat);
}

pub fn tuple() {
  let tup: (i32, f64, u8) = (500, 6.4, 1);

  println!("tuple: {:#?}", tup);
}

pub fn tuple2() {
  let tup = (500, 6.4, 1);

  let first = tup.0;
  let second = tup.1;
  let third = tup.2;

  println!("tuple2: {} {} {}", first, second, third);
}

/** 배열 */
pub fn array() {
  let a = [1, 2, 3, 4, 5];
  // println!("array: {:#?}", a);
  for el in a.iter() {
    print!("array el: {}", el)
  }

  let first_el = a[0];
  let second_el = a[1];
  println!("{} {}", first_el, second_el);
}