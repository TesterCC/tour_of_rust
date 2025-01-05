/*
https://tourofrust.com/06_zh-cn.html
https://tourofrust.com/06_en.html

基本类型转换 Basic Type Conversion

当涉及到数字类型时，Rust 要求明确。一个人不能想当然地把“u8”用在“u32”上而不出错。

幸运的是，使用 as 关键字，Rust 使数字类型转换非常容易。
*/

fn main() {
    let a = 13u8;
    let b = 7u32;
    let c = a as u32 + b;
    println!("{}", c);

    let t = true;
    println!("{}", t as u8); // bool can trans to num
}





