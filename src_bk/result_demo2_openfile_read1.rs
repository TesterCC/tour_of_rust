/*

from ChatGPT

Result<T, E> 是什么？

Result<T, E> 是 Rust 用来处理 可能失败的操作 的枚举类型（enum），它用于表示操作的结果可能是成功，也可能是失败。

enum Result<T, E> {
    Ok(T),  // 表示成功，包含返回值 T
    Err(E), // 表示失败，包含错误信息 E
}

*/

// example2:文件读取, read file and print
use std::fs::File;
use std::io::{self, Read};

// method1 使用 read_to_string() 读取整个文件
fn read_file_content(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?; // 打开文件
    let mut content = String::new();
    file.read_to_string(&mut content)?; // 读取内容
    Ok(content) // 返回内容
}

fn main() -> Result<(), io::Error> {
    let content = read_file_content("src_bk/test.txt")?;
    println!("文件内容:\n{}", content);
    Ok(())
}

