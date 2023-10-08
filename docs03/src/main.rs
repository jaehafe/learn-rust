
mod let_mut;
mod data_types;
mod function;
mod control;
mod ownership;
mod ownership_1;
mod ownership_2;
mod references_borrowing;
mod slices;
mod program_using_structs;
mod method_syntax;
mod structure;
mod defining_enum;

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

    ownership::main();
    ownership_1::main();
    ownership_2::main();

    references_borrowing::main();
    slices::main();

    structure::main();
    program_using_structs::main();
    method_syntax::main();

    defining_enum::main();
}

fn plus_test() {
    let x = plus_one(5);
    println!("main: {}", x);
}
fn plus_one(x: i32) -> i32 {
    x + 1
}