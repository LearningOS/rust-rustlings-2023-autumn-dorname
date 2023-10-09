// variables2.rs
//
// Execute `rustlings hint variables2` or use the `hint` watch subcommand for a
// hint.

// DONE

fn main() {
    // let x:i32 = 0;
    // 变量声明必须声明类型和初始值
    // 给了初始值会自动推导类型
    let mut x:i32 = 0;
    x = 10;
   
    if x == 10 {
        println!("x is ten!");
    } else {
        println!("x is not ten!");
    }
}
