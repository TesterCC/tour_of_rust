/*
https://www.bilibili.com/video/BV1J7Q9YQEqt/

a basic rust demo

// Rust的Vec，类似于 Python 的 list 或 Go 的 slice，但有区别
*/

fn main() {
    let mut hello: Vec<i32> = (0..10).collect();

    fn do_stuff(val: &Vec<i32>) {
        // println!("{:?}", val.len()); // 10
        println!("{}", val.len());  // 10
    }

    do_stuff(&hello);

    println!("--------------------");
    println!("{:?}", hello);

}
