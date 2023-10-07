extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1, 101);
        // .to_string();

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");
    
        // 새로운 빈 String 인스턴스와 연결된 가변변수를 생성
        let mut guess = String::new(); 

        io::stdin().read_line(&mut guess)
            .expect("failed to read line");

            // u32 부호가 없는 32비트 정수
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
            // .expect("Please type a number!");

        println!("you guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You win!");
                break;
            }
        }
    }
}