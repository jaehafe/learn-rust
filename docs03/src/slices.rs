pub fn main() {
  println!();
  println!("slices!!");

  fn1();
  // fn2();
}

fn fn1() {
  let mut s = String::from("hello world");
  let word = first_word(&s);
  println!("word: {}", word);
  
  s.clear();
}

fn first_word(s: &String) -> &str {
  let bytes = s.as_bytes();
  // println!("bytes: {:#?}", bytes);

  for (i, &item) in bytes.iter().enumerate() {
      if item == b' ' {
          return &s[0..i];
      }
  }

  &s[..]
}

// fn fn2() {
//   let s = String::from("hello world");

//   let len = s.len();

//   let slice = &s[0..len];
//   let slice1 = &s[..];

//   println!("{} {}", slice, slice1);
// }