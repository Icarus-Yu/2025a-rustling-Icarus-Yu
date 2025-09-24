// macros1.rs
//
// Execute `rustlings hint macros1` or use the `hint` watch subcommand for a
// hint.


macro_rules! my_macro {
    // 规则 1: 匹配不带参数的调用
    () => {
        println!("Check out my macro!");
    };
    // 规则 2: 匹配带一个表达式参数的调用
    ($val:expr) => {
        println!("Look at this expression: {}", $val);
    };
}

fn main() {
    // 这会匹配第一条规则
    my_macro!();
    
    // 这会匹配第二条规则
    my_macro!(1 + 1);
    my_macro!("hello world");
}