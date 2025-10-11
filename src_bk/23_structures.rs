/*
https://tourofrust.com/23_zh-cn.html
https://tourofrust.com/23_en.html

Chapter 3 - Basic Data Structure Types (start)  

结构体

一个 struct 就是一些字段的集合。

字段是一个与数据结构相关联的数据值。它的值可以是基本类型或结构体类型。

它的定义就像给编译器的蓝图，告诉编译器如何在内存中布局彼此相邻的字段。

*/

// e.g., can't run directly
struct SeaCreature {
    // String 是个结构体
    animal_type: String,
    name: String,
    arms: i32,
    legs: i32,
    weapon: String,
}
