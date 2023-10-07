
mod let_mut;
mod data_types;
mod function;
mod control;


fn main() {
    plus_test();
    let_mut::main();
    let_mut::main1();
    let_mut::main2();

    data_types::main1();
    data_types::tuple2();
    data_types::array();

    function::main();
    
    control::main();
}

fn plus_test() {
    let x = plus_one(5);
    println!("main: {}", x);
}
fn plus_one(x: i32) -> i32 {
    x + 1
}