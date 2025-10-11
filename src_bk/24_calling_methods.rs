/*
https://tourofrust.com/24_zh-cn.html
https://tourofrust.com/24_en.html

Chapter 3 - Basic Data Structure Types 第三章 - 基本数据结构类型

方法调用

与函数（function）不同，方法（method）是与特定数据类型关联的函数。

静态方法 — 属于某个类型，调用时使用 :: 运算符。   p.s. 类似python class中的@staticmethod的定义

实例方法 — 属于某个类型的实例，调用时使用 . 运算符。

我们将在后续章节中更多地讨论如何创建自己的方法。

*/

fn main() {
    // 使用静态方法来创建一个String实例
    let s = String::from("Hello world, Rust!");
    // 使用实例来调用方法 s.len()
    println!("{} is {} characters long.", s, s.len());
}

