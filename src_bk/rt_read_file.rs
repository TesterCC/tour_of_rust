/*
Rust小技巧：读文件
https://www.bilibili.com/video/BV1xuwHePESi/

simple read file

attention: rust string default use encoding utf-8
*/
use std::fs;

fn main() -> Result<(),std::io::Error> {
    let content = fs::read_to_string("src_bk/test.txt")?;  // root dir is project ./
    println!("{}", content);
    Ok(())
}






