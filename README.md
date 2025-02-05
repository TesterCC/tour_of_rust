# 项目介绍

https://tourofrust.com/ 的联系demo，快速熟悉rust基本语法

After learning, Reading https://doc.rust-lang.org/stable/book/  (B站有该书配套视频)

## 运行问题

### 1.使用IDE
如果用RustRover等IDE来开发调试，建议最好是用`cargo new project_name`来构建一个基本项目，然后用IDE打开

### 2.使用命令行
如果只需要调试很简单的脚本，可以在terminal中进行。
```shell
# 单.rs（rust script）执行方式
rustc xxx.rs
./xxx
```

## 项目结构

- `/src/main.rs` 由Cargo创建项目时生成，程序的入口，这里专门用于调试x.rs的入口文件。
- `/src_bk`中是已练习万的脚本，如果要在本项目中运行，需要把内容复制粘贴到`../src/main.rs`中保存，然后运行。
