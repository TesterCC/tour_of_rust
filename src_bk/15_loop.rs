/*
https://tourofrust.com/15_zh-cn.html
https://tourofrust.com/15_en.html

循环 Loop

需要一个无限循环？

使用 Rust 很容易实现。

break 会退出当前循环。但 loop 还有个秘密，我们很快讲到。

*/

fn main() {
    let mut x = 0;
    loop {
        x += 1;
        // if comment x==77 and cancel break, while cause infinite loop
        if x == 77 {
            break;
        }
    }
    println!("current x: {}", x);
}

