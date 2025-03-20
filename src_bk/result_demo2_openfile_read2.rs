/*

from ChatGPT

Result<T, E> 是什么？

Result<T, E> 是 Rust 用来处理 可能失败的操作 的枚举类型（enum），它用于表示操作的结果可能是成功，也可能是失败。

enum Result<T, E> {
    Ok(T),  // 表示成功，包含返回值 T
    Err(E), // 表示失败，包含错误信息 E
}


? 在 Rust 中是 错误传播（Error Propagation） 运算符，用于简化 Result<T, E> 和 Option<T> 的错误处理。

作用
	1.	如果结果是 Ok(value)，提取 value 并继续执行。
	2.	如果结果是 Err(e)，直接返回 Err(e)，并提前结束当前函数。

*/

// example2:文件读取, read file and print
use std::fs::File;
use std::io::{self, BufRead, BufReader};

// method2 使用 read_to_string() 使用 BufReader 按行读取
// 如果文件很大，read_to_string() 会占用大量内存，因此可以用 BufReader 按行读取

fn read_file_by_lines(filename: &str) -> Result<(), io::Error> {
    let file = File::open(filename)?; // 打开文件
    let reader = BufReader::new(file); // 创建缓冲读取器

    for line in reader.lines() {
        println!("{}", line?); // 逐行读取并打印
    }

    Ok(())
}

fn main() -> Result<(), io::Error> {
    let languages= ["python", "rust", "go", "java", "javascript", "php"];
    println!("{:?}", languages);
    println!("{:#?}", languages); // 美化版{:?}，格式化输出，适合复杂数据
    println!("-------------------------------------------");

    read_file_by_lines("src_bk/test.txt")?;
    Ok(())

}

