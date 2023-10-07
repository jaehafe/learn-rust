mod main1;
mod main3_1;

fn main() {
    plus_test();
    main1::main1();
    main3_1::main();
    main3_1::main1();
    main3_1::main2();
}

fn plus_test() {
    let x = plus_one(5);
    println!("main: {}", x);
}
fn plus_one(x: i32) -> i32 {
    x + 1
}
