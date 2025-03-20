/*

from ChatGPT

Result<T, E> 是什么？

Result<T, E> 是 Rust 用来处理 可能失败的操作 的枚举类型（enum），它用于表示操作的结果可能是成功，也可能是失败。

enum Result<T, E> {
    Ok(T),  // 表示成功，包含返回值 T
    Err(E), // 表示失败，包含错误信息 E
}

*/

// example1:除非操作

fn divide(x: f64, y: f64) -> Result<f64, String> {
    if y == 0.0 {
        Err("除数不能为零".to_string()) // 返回错误
    } else {
        Ok(x / y) // 返回结果
    }
}


fn main() {
    // Rust 的 match 语法类似 switch，但更强大，可以匹配枚举、结构体、模式绑定等。  如果只关心 Ok 分支，可以使用 if let
    match divide(10.0, 2.0) {
        Ok(result) => println!("结果: {}", result),
        Err(err) => println!("错误: {}", err),
    }

    match divide(10.0, 0.0) {
        Ok(result) => println!("结果: {}", result),
        Err(err) => println!("错误: {}", err),
    }

}

