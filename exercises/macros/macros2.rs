// macros2.rs
//
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a
// hint.


// macros2.rs

// 先定义宏
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

// 再使用宏
fn main() {
    my_macro!();
}