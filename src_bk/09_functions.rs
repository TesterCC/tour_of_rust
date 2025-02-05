/*
https://tourofrust.com/09_zh-cn.html
https://tourofrust.com/09_en.html

函数 Functions

函数可以有 0 个或者多个参数。

在这个例子中，add 接受类型为 i32（32 位长度的整数）的两个参数。

函数名总是遵循 蛇形命名法 (snake_case)。
*/

// custom function add()
fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

fn main() {
    println!("{}", add(43, 34));

}
