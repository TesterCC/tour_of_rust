/*
https://tourofrust.com/14_zh-cn.html
https://tourofrust.com/14_en.html

if/else

Rust 中的代码分支不足为奇。

Rust 的条件判断没有括号！需要括号干什么?我们现有的逻辑就看起来就很干净整洁呀!

不过呢，所有常见的逻辑运算符仍然适用：==，!=， <， >， <=， >=， !， ||， &&

*/
use std::io;

fn main() {
    // new, improve x, x value from user input
    println!("Please input a number: ");
    let mut input_data = String::new();
    io::stdin().read_line(&mut input_data).expect("Failed to read line");

    let x: i32 = input_data.trim().parse().expect("Please type a number!");

    // origin
    // let x = 77;  // default i32

    if x < 77 {
        println!("less than 77");
    } else if x == 77 {
        println!("equal to 77");
    } else {
        println!("greater than 77");
    }
}

