/*

from ChatGPT

Result<T, E> 是什么？

Result<T, E> 是 Rust 用来处理 可能失败的操作 的枚举类型（enum），它用于表示操作的结果可能是成功，也可能是失败。

enum Result<T, E> {
    Ok(T),  // 表示成功，包含返回值 T
    Err(E), // 表示失败，包含错误信息 E
}

*/

// example2:文件读取, read file
use std::fs::File;
use std::io::Error;

// 接收一个字符串切片（Rust 中的不可变引用 &str，类似 Go 的 string
fn open_file(filename: &str) -> Result<File, Error> {
    File::open(filename) // 返回 Result<File, Error>
}

fn main() {
    match open_file("src_bk/test.txt") {
        Ok(file) => println!("文件打开成功: {:?}", file),  // File { fd: 3, path: "/Users/seccodecat/Development/ws_rust/tour_of_rust/src_bk/test.txt", read: true, write: false }
        Err(e) => println!("文件打开失败: {}", e),
    }
}

