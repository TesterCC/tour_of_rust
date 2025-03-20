/*
https://tourofrust.com/18_zh-cn.html
https://tourofrust.com/18_en.html

match

想念你的 switch 语句吗？Rust 有一个非常有用的关键字，用于匹配值的所有可能条件，并在匹配为真时执行相应代码。

我们先来看看对数字的使用。在未来章节中，我们将有更多 更复杂的数据模式匹配的说明，我向你保证，它将值得等待。

match 是穷尽的，意为所有可能的值都必须被考虑到。

匹配与解构相结合是迄今为止你在 Rust 中看到的最常见的模式之一。

*/

fn main() {
    // let x = 4200;
    let x = 77;
    // 类似switch
    match x {
        0 => {
            println!("found zero");
        }
        // 我们可以匹配多个值
        1 | 2 => {
            println!("found 1 or 2!");
        }
        // 我们可以匹配迭代器
        3..=9 => {
            println!("found a number 3 to 9 inclusively");
        }
        // 在 Rust 语言中，@用于模式绑定，允许你在匹配模式时，给匹配到的值绑定一个名称，以便在匹配分支中使用。
        // 我们可以将匹配数值绑定到变量，当在 10 到 100 之间时，matched_num 绑定该值。
        // matched_num @ 10..=100 是一个模式。10..=100 定义了数值范围，matched_num 则是绑定的名称。
        // 当匹配到的值在 10 到 100 这个范围内时，该值会被绑定到 matched_num 变量上，这样在匹配分支 { println!("found {} number between 10 to 100!", matched_num); } 中就可以使用 matched_num 了。
        matched_num @ 10..=100 => {
            // 这里 matched_num 是匹配到的值，它的范围是 10 到 100（包括 10 和 100）
            println!("found {} number between 10 to 100!", matched_num);
        }
        // default match, 这是默认匹配，如果没有处理所有情况，则必须存在该匹配
        _ => {
            println!("found something else!");
        }
    }
}

