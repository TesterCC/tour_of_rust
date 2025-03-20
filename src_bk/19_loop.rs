/*
https://tourofrust.com/19_zh-cn.html
https://tourofrust.com/19_en.html

从循环中返回值
loop 可以被中断以返回一个值。
loop can break to return a value.
*/

fn main() {
    let mut x = 0;
    let v = loop {
        x += 1;
        if x == 13 {
            break "found the 13";
        }
    };
    println!("from loop: {}", v);
}
