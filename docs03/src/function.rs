pub fn main() {
    println!("function!!!");
    another_fn(5);
    fn2();
    fn4();
    fn5();
}

fn another_fn(x: i32) {
    println!("value: {}", x)
}

fn fn2() {
  let _x = 5;
  let y = {
    let x = 3;
    x + 1
  };

  println!("fn2 return: {}", y);
}


fn fn3() -> i32 {
  5
}

fn fn4() {
  let x = fn3();

  println!("fn3 return: {}", x)
}

fn fn5() {
  let x = plus_one(5);

  println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {

  x + 1
}