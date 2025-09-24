// macros3.rs
//
// Make me compile, without taking the macro out of the module!
//
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a
// hint.



// macros3.rs

// 通过 #[macro_use] 属性，我们将 macros 模块中定义的宏导入到当前作用域
#[macro_use]
mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    // 现在 my_macro! 在 main 函数的作用域内是可见的了
    my_macro!();
}