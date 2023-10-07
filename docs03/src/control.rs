pub fn main() {
  fn1();
  fn2();
  fn3();
  fn4();
  fn5();
  while_fn();
  for_fn();
  iter_fn();
}

fn fn1() {
  println!("제어문!!!");

  let number = 3;

  if number < 5 {
    println!("condition was true");
  } else {
    println!("condition was false");
  }
}

fn fn2() {
  let number = true;

  if number {
    println!("true")
  } else {
    println!("false")
  }
}

fn fn3() {
  let number = 6;

  
  if number % 4 == 0 {
    println!("number is divisible by 4");
  } else if number % 3 == 0 {
      println!("number is divisible by 3");
  } else if number % 2 == 0 {
      println!("number is divisible by 2");
  } else {
      println!("number is not divisible by 4, 3, or 2");
  }
}

fn fn4() {
    let condition = true;

    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
}

enum Number {
  Integer(i32),
  Text(&'static str),
}

fn fn5() {
  let condition = true;

  let number = if condition {
      Number::Integer(5)
  } else {
      Number::Text("six")
  };

  match number {
      Number::Integer(i) => println!("The value of number is an integer: {}", i),
      Number::Text(s) => println!("The value of number is text: {}", s),
  }
}


fn while_fn() {
  let mut number = 3;

  while number != 0 {
    println!("{}!", number);
    
    number = number - 1
  }

  println!("LIFTOFF")
}

fn for_fn() {
  let a = [10, 20, 30, 40, 50];
  let mut index = 0;

  while index <  5 {
    println!("for fn: {}", a[index]);


    index = index + 1;
  }
}

fn iter_fn() {
  // rev: range 역순
  for number in (1..4).rev() { 
    println!("iter_fn {}!", number);
  }

  println!("LIFTOFF!");
}