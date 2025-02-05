/*
https://tourofrust.com/10_zh-cn.html
https://tourofrust.com/10_en.html

多个返回值 Multiple Return Values

函数可以通过元组来返回多个值。

元组元素可以通过他们的索引来获取。

Rust 允许我们将后续会看到的各种类型的解构，也允许我们以符合逻辑的方式提取数据结构的子片段。
Rust supports various kinds of destructuring that we will see in many forms, allowing us to extract sub-pieces of data structures in ergonomic ways

*/

fn swap(x: i32, y: i32) -> (i32, i32) {
    // return (y, x);  // return can omit
    (y, x)  // 函数可以通过元组来返回多个值。
}

fn main() {
    // return a tuple of return values, 返回一个元组
    let result = swap(123, 321);
    println!("{} {}", result.0, result.1);

    // destructure the tuple into two variables names
    // 将元组解构为2个变量；元组元素可以通过他们的索引来获取。
    let (a, b) = swap(result.0, result.1);  // 和Python的tuple不同
    println!("{} {}", a, b);
}

