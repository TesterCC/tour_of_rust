/*
https://tourofrust.com/20_zh-cn.html
https://tourofrust.com/20_en.html

从块表达式返回值

if，match，函数，以及作用域块都有一种返回值的独特方式。

如果 if、match、函数或作用域块中的最后一条语句是不带 ; 的表达式， Rust 将把它作为一个值从块中返回。

这是一种创建简洁逻辑的好方法，它返回一个 可以放入新变量的值。

注意，它还允许 if 语句像简洁的三元表达式一样操作。
*/

// -> i32指定该函数返回一个32位有符号整数类型
fn example() -> i32 {
    let x = 42;
    // Rust's ternary expression  Rust的三元表达式
    // 这里的if不是一个单纯的流程控制语句，而是一个表达式，它会根据条件判断产生一个值，并可以直接赋值给变量v
    // Rust要求if和else分支返回值的类型必须相同，否则会导致编译错误
    let v = if x < 42 { -1 } else { 1 };
    println!("from if: {}", v);

    let food = "hamburger";

    // match是Rust强大的模式匹配工具，它会将food的值与一系列模式进行比较，执行第一个匹配的模式对应的代码块
    let result = match food {
        // 当food的值为"hotdog"时，会执行=>右侧的表达式"is hotdog"
        "hotdog" => "is hotdog",
        // notice the braces are optional when its just a single return expression
        // 注意，当它只是一个返回表达式时，大括号是可选的
        // _是一个通配符，会匹配所有其他可能的值
        // 当food是其它值时，会执行=>右侧的表达式"is not hotdog"
        _ => "is not hotdog",
        // Rust强制要求match表达式必须覆盖所有可能的情况，如果漏掉一些情况，编译器会报错。使用_通配符是处理剩余情况的常见方法。
    };
    println!("identifying food: {}", result);

    // 由一对花括号{}包围的代码块也是一个表达式。这个块会创建一个新的作用域，其最后一个表达式的值（此处是a + b）会成为整个块表达式的值，并赋值给变量v。
    let v = {
        // This scope block lets us get a result without polluting function scope
        // 这个作用域块让我们得到一个不影响函数作用域的结果
        let a = 1;
        let b = 2;
        a + b
    };
    println!("from block: {}", v);

    // The idiomatic way to return a value in rust from a function at the end
    // 在最后从函数中返回值的惯用方法; 函数example的最后一行是一个表达式v + 4（没有分号），它的值将自动作为整个函数的返回值。
    v + 4
}


fn main() {
    println!("from function: {}", example());
}

/*
from if: 1
identifying food: is not hotdog
from block: 3
from function: 7
*/