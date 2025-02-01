/*
https://tourofrust.com/07_zh-cn.html
https://tourofrust.com/07_en.html

常量 Constants

常量允许我们高效地指定一个在代码中会被多次使用的公共值。不同于像变量一样在使用的时候会被复制，常量会在编译期间直接用它们的值来替换变量的文本标识符。

不同于变量，常量必须始终具有显式的类型。

常量名总是遵循 全大写蛇形命名法（SCREAMING_SNAKE_CASE）。
*/

// const PI: f32 = 3.141592653; // warning
// const PI: f32 = std::f32::consts::PI;
const PI: f64 = std::f64::consts::PI;

fn main() {
    println!(
        "To make an apple {} from scratch, you must first create a universe.",
        PI
    );
}
