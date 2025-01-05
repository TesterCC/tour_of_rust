/*
https://tourofrust.com/05_zh-cn.html
https://tourofrust.com/05_en.html

基本类型

Rust 有多种常见的类型：

布尔型 - bool 表示 true 或 false
无符号整型- u8 u32 u64 u128 表示非负整数
有符号整型 - i8 i32 i64 i128 表示整数
指针大小的整数 - usize isize 表示内存中内容的索引和大小
浮点数 - f32 f64
元组（tuple） - (value, value, ...) 用于在栈上传递固定序列的值
数组 (array) - [value, value, ...] 在编译时已知的具有固定长度的相同元素的集合
切片（slice） - 在运行时已知长度的相同元素的集合
str(string slice) - 在运行时已知长度的文本
文本可能比你在其他语言中学到的更复杂，因为 Rust 是一种系统编程语言，它关心的是你可能不太习惯的内存问题。
我们之后将详细讨论这个问题。

另外，你也可以通过将类型附加到数字的末尾来明确指定数字类型（如 13u32 和 2u8）

*/

fn main() {
    let x = 12; // 默认情况下，这是i32
    let a = 12u8; // 将类型附加到数字的末尾来明确指定数字类型
    let b = 4.3; // 默认情况下，这是f64
    let c = 4.3f32;  // 将类型附加到数字的末尾来明确指定数字类型
    let bv = true;   // 布尔型
    let t = (13, false);  // 元组tuple，包含两个元素，第一个元素是整数 13，类型是 i32（Rust 默认整型）；第二个元素是布尔值 false，类型是 bool。
    let sentence = "hello world!";
    println!(
        "{} {} {} {} {} {} {} {}",
        x, a, b, c, bv, t.0, t.1, sentence   // 注意tuple元素的调用方式
    );
}




