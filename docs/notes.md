## Rust-for-Malware-Development

blog:
https://maldev.5mukx.site/

github:
https://github.com/Whitecat18/Rust-for-Malware-Development


### Table of Contents

- Walkthrough
- Malware Techniques
- Encryption Techniques
- Related Blogs
- Download Repository
- Contribution
- Credits/References

### Malware Techniques

https://maldev.5mukx.site/#malware-techniques

### Encryption Techniques

https://maldev.5mukx.site/#encryption-techniques

### others
https://x.com/5mukx/status/1930881788945281437
https://5mukx.site/blog/programming-for-infosec


### Rust for Cyber Security and Red Teaming

ref:

[Rust for Cyber Security and Red Teaming](https://infosecwriteups.com/rust-for-cyber-security-and-red-teaming-275595d3fdec)

ext:

掌握 Rust 如何助力您的网络安全职业发展？

掌握 Rust 可以显著提升您的网络安全职业发展，原因如下：

内存安全：Rust 是一种优先考虑内存安全的系统编程语言。它对内存访问方式实施严格的规则，防止常见的漏洞，例如缓冲区溢出、空指针引用和其他内存相关错误。在网络安全领域，这些漏洞经常被攻击者利用来入侵系统。使用 Rust，您可以降低代码中引入此类漏洞的风险，从而提高软件的安全性。

性能和效率：Rust 专注于零成本抽象和对系统资源的低级控制，这使得开发人员能够构建高性能应用程序。在性能和效率至关重要的网络安全领域，Rust 是开发工具、应用程序甚至安全基础设施组件的绝佳选择。

并发和并行：Rust 拥有强大的并发模型，可以更轻松地编写并发和并行代码，而不会引入数据竞争和其他线程问题。在网络安全领域，任务通常需要并行处理和处理多个连接，Rust 的并发特性有助于构建健壮高效的系统。

跨平台兼容性：Rust 的设计强调跨平台兼容性，允许开发人员编写可轻松移植到不同操作系统和架构的代码。这在网络安全领域尤为重要，因为网络安全需要分析和保护不同的环境和系统。

社区和工具支持：Rust 社区活跃、充满活力且注重安全。因此，Rust 中与安全相关的库、工具和框架的生态系统正在不断发展。利用这种支持可以加速您的开发流程，并帮助您构建更安全的应用程序。

代码审查和审计：Rust 对可读性和可维护性的高度重视，在代码审查和安全审计过程中非常有利。简洁、富有表现力的代码更易于理解和分析，从而更容易识别潜在的安全漏洞和漏洞。

漏洞赏金计划：许多公司和组织为发现并负责任地披露其软件漏洞的安全研究人员提供漏洞赏金计划。通过展示您对 Rust 的熟练掌握并在基于 Rust 的项目中发现安全问题，您可以参与这些计划并获得网络安全社区的认可。

渗透测试和安全研究：Rust 可用于开发用于渗透测试、漏洞分析和安全研究的安全工具和框架。通过贡献或创建此类工具，您可以成为网络安全领域知识渊博、技术娴熟的专业人士。

总而言之，掌握 Rust 可以提升您的技能，并在网络安全行业中提升您的竞争优势。它对安全性、性能和并发性的关注与现代网络安全挑战的需求完美契合，使其成为该领域专业人士的理想语言选择。


## Rust SecDev项目

https://github.com/bee-san/RustScan     # 端口快速扫描
https://github.com/epi052/feroxbuster   # 内容发现
https://github.com/AFLplusplus/LibAFL  # 模糊测试
https://github.com/m4b/goblin     # 二进制分析
https://github.com/kpcyrd/sniffglue  # 安全的多线程数据包嗅探器


## Rust Roadmap for Cyber Security

### MODULE 1 : <基础>

- [Rust Book with Quiz](https://rust-book.cs.brown.edu/ch00-00-introduction.html): An unofficial documentation (an copy of Rust Book ) that has quizzes and question per topic ( Recommend Document).

布朗大学认知实验，基于官方的Rust Book的内容做的Rust入门课程。

- [Rustlings](https://github.com/rust-lang/rustlings/)

- 对于练习，我强烈推荐 Rustlings，这是一个通过解决问题来教授 Rust 的 Rust 程序“通过实践学习 Rust”。

### MODULE 2 : <基础\中级>

p.s. 内容有点多，选择性学习吧

- [Microsoft’s Path](https://learn.microsoft.com/en-us/training/browse/) - 虽然有人强烈推荐，但这个需要自己找资源。 An complete path to become strong at Basics and Intermediate. ( Highly Recommend )

- [Rust By Example](https://doc.rust-lang.org/stable/rust-by-example/) - An Mini Documentation that Explains Rust with runnable examples that illustrate various Rust concepts and standard libraries ( Recommend )

- [Rust Macros](https://earthly.dev/blog/rust-macros/#:~:text=In%20Rust%2C%20macros%20are%20pieces,of%20a%20macro%20is%20println!%20) - Practical Examples and Best Practices. ( Highly Recommend )

- [Asynchronous Programming](https://rust-lang.github.io/async-book/) - Learn the Basics of multi-threading programming. ( Highly Recommend )

- [Windows API](https://kennykerr.ca/rust-getting-started/) - Learn how to implement windows API with Rust. ( Highly Recommend )


### MODULE 3: <中级\高级>

进入底层开发阶段

- [Writing OS in Rust](https://os.phil-opp.com/) - Since you do not need to write the Kernel in rust but read the fundamentals like Heap Allocation , Async/Await , Paging . It the most common thing to learn in rust if you are coding an system level tools and projects like `Mimikaz` etc.

- [Rust Atomics and Locks](https://marabos.nl/atomics/preface.html) - Learn low-level concurrency looks like from a Rust perspective. Great book to learn about threads ,mutexes, references , interior mutability , memory ordering etc.

- [OffensiveRust](https://github.com/trickster0/OffensiveRust)

- Web框架：[Actix](https://actix.rs/)（持续迭代中，优先学这个） 或 [Rocket](https://rocket.rs/)（进一年未更新，2025年最近一次更新是2024年5月）

### Additional

- OffensiveRust: Rust Weaponization for Red Team Engagements.
  https://github.com/trickster0/OffensiveRust

- black-hat-rust
  https://github.com/skerkour/black-hat-rust

- awesome-rust-security
  https://github.com/osirislab/awesome-rust-security#offensive-security-and-red-teaming

- Security Related Rust Projects
  https://github.com/rust-secure-code/projects

### Top Books for Rust to Master

- [**Black Hat Rust**](https://kerkour.com/black-hat-rust) by Sylvain Kerkour -- 必看
- [**Network Programming with Rust**](https://www.amazon.com/Network-Programming-Rust-memory-safety-concurrency/dp/1788624890) by Abhishek Chanda
- [**Zero to Production**](https://www.zero2prod.com/index.html?country=India&discount_code=SEA60) in Rust by Luca Palmieri  -- 有中文版，而且推荐者较多
- [**Command Line Rust**](https://www.amazon.com/Command-Line-Rust-Ken-Youens-Clark-ebook/dp/B09QFQ3Y64?ref_=ast_author_dp) by y [Ken Youens-Clark](https://www.amazon.in/Ken-Youens-Clark/e/B08DDCNNL3/ref=dp_byline_cont_book_1)
- [**Hands-On System Programming with Rust**](https://www.goodreads.com/book/show/58429656-hands-on-systems-programming-with-rust) by Ken Youens-Clark
- [**Rust Servers, Services and Apps**](https://www.amazon.com/Rust-Servers-Services-Prabhu-Eshwarla/dp/1617298603) by Prabhu Eshwarla
- [**Rust in Action**](https://www.amazon.com/Rust-Action-Tim-McNamara/dp/1617294551) by Tim McNamara
- [**Practical System Programming**](https://www.amazon.com/Practical-System-Programming-Rust-Developers/dp/1800560966) for Rust Developers by by [Prabhu Eshwarla](https://www.amazon.com/Prabhu-Eshwarla/e/B08RBSMC5F/ref=dp_byline_cont_book_1)

### Extra Tips for Learning Rust

