/*
https://tourofrust.com/16_zh-cn.html
https://tourofrust.com/16_en.html

while

while 允许你轻松地向循环添加条件。

如果条件一旦变为 false，循环就会退出。

*/

fn main() {
    let mut x = 0;
    while x != 77 {
        x += 1;
        // println!("tmp x: {}", x); // debug print
    }
    println!("x is {}", x);
}

