/*
https://tourofrust.com/11_zh-cn.html
https://tourofrust.com/11_en.html

返回空值  Returning Nothing

如果没有为函数指定返回类型，它将返回一个空的元组，也称为单元Unit。
If no return type is specified for a function, it returns an empty tuple, also known as a unit.

一个空的元组用 () 表示。

直接使用 () 的情况相当不常见。但它经常会出现（比如作为函数返回值），所以了解其来龙去脉非常重要。
*/

fn make_nothing() -> () {
    return ();
}

// 返回类型隐含为 ()
fn make_nothing2() {
    // 如果没有指定返回值，这个函数将会返回 ()
}

fn main() {
    let a = make_nothing();
    let b = make_nothing2();

    // 打印a和b的debug字符串，因为很难去打印空 {:?} 用于打印调试信息
    // 在 Rust 中，{:?} 是 调试（debug）格式化 的占位符，用于 println!() 打印实现了 Debug trait 的类型。
    // 它用于 打印结构体、枚举、元组等复杂数据类型，方便调试。
    println!("The value of a: {:?}", a);
    println!("The value of b: {:?}", b);

}

