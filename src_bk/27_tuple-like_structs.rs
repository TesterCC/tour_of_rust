/*
https://tourofrust.com/27_zh-cn.html
https://tourofrust.com/27_en.html

Chapter 3 - Basic Data Structure Types 第三章 - 基本数据结构类型

类元组结构体  Tuple-like Structs
简洁起见，你可以创建像元组一样被使用的结构体。

*/


struct Location(i32, i32);

fn main() {
    // 这仍然是一个在栈上的结构体
    let loc = Location(52, 90);
    println!("GPS location is: {} N, {} E", loc.0, loc.1);
}

