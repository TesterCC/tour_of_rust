/*
https://tourofrust.com/04_zh-cn.html

Rust 非常关心哪些变量是可修改的。值分为两种类型：

可变的 - 编译器允许对变量进行读取和写入。
不可变的 - 编译器只允许对变量进行读取。
可变值用 mut 关键字表示。
*/
fn main() {
    let mut x = 66; // mut means mutable value, 可变值
    println!("{}", x);
    x = 15;
    println!("{}", x);

    let y = 77; // 定义不可变值，省略mut关键字即可，因为系统变量默认是不可变值immutable
    println!("{}", y);
    // y = 88;  // 直接有语法提示报错
}




