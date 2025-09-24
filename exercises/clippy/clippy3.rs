// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.



#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    // 修正1: 删除了总会 panic 的 if 块。
    // if my_option.is_none() {
    //     my_option.unwrap();
    // }

    let my_arr = &[
        -1, -2, -3, // 修正2: 添加了逗号
        -4, -5, -6,
    ];
    println!("My array! Here it is: {:?}", my_arr);

    // 修正3: 使用 Vec::new() 创建一个空的 Vec。
    let my_empty_vec: Vec<i32> = Vec::new(); 
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // 修正4: 使用 std::mem::swap 来正确交换变量。
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}