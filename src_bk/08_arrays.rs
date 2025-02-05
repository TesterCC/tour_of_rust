/*
https://tourofrust.com/08_zh-cn.html
https://tourofrust.com/08_en.html

数组 Arrays -- 类似Python list

数组是所有相同类型数据元素的固定长度集合。

一个数组的数据类型是 [T;N]，其中 T 是元素的类型，N 是编译时已知的固定长度。

可以使用 [x] 运算符提取单个元素，其中 x 是所需元素的 usize 索引（从 0 开始）。
*/

fn main() {
    let nums: [i32; 3] = [1, 2, 3];
    // Rust 数组的长度是固定的，不能动态调整。Rust 的字符串字面量 &str 是字符串切片，因此数组的类型是 [&str; 3]
    let languages= ["python", "rust", "go", "java", "javascript", "php"];
    println!("{:?}", nums);   // [1,2,3]
    println!("{}", nums[1]);  // 2
    println!("{}", nums[2]);  //

    // {:?} 用于打印数组的调试信息
    println!("{:?}", languages);
    println!("second is {}, third is {}", languages[1], languages[2]);
}
