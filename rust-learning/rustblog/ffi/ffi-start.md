## [【Rust每周一知】Rust FFI 开发系列教程 - 序言](https://rust.cc/article?id=44a47b42-184c-4e2f-9318-be07ee5a516c)

[Mike Tang](https://rust.cc/blog_with_author?author_id=09e42b7c-c2bc-410a-9079-8ad0370d2603)  发表于  2020-03-26 12:23

Tags：rust

本章为《Rust FFI 开发系列教程》的序言。

### Rust FFI

FFI (Foreign Function Interface) 是 Rust 中非常重要的一个部分。Rust 本身的一个定位——系统开发的能力——赋予它（FFI）在Rust整个生态中一个特殊的地位。

FFI 主要在两个大的领域起作用：

1.  系统开发；
2.  跨语言开发。

### 系统开发

我们此处定义的系统开发，有较丰富的意义，包括但不限于：

-   对 OS 系统调用的封装；
-   对系统底层 C 库的封装；
-   广泛平台的嵌入式/IoT 开发；
-   OS 开发；
-   充分利用现代硬件平台的性能优势；

### 跨语言开发

跨语言开发的意思是，可以使用 Rust 语言为其它（几乎任何）语言写调用库。主要分为：

-   利用 C ABI 进行跨语言调用；
-   利用 Wasm 进行跨语言调用；

### 本系列教程的大纲

1.  FFI 的基本概念和基本原理
2.  Rust 标准库中对 FFI 支持的基础设施准备
3.  使用 libc crate 进行 FFI 增强支持
4.  使用 nix crate 进行类 unix 系统开发
5.  使用 rust-bindgen 进行自动 C 库封装，生成 *-sys crate
6.  Rust 对不同硬件平台的支持
7.  Rust 到各平台的交叉编译
8.  使用 Rust FFI 进行嵌入式平台开发
9.  使用 Rust FFI 进行 GPU 编程
10.  使用 Rust FFI 进行高性能计算编程
11.  使用 Rust FFI 为 C/C++ 写调用库（跨平台开发）
12.  使用 cbindgen 为 C/C++ 自动生成调用库头文件
13.  使用 Rust FFI 为 Python 写调用库
14.  使用 Rust FFI 为 Ruby 写调用库
15.  使用 Rust FFI 为 Nodejs 写调用库
16.  使用 Rust FFI 为 Php 写调用库
17.  使用 Rust FFI 为 Go 写调用库
18.  使用 Rust FFI 为 Java 写调用库
19.  使用 Rust + Wasm 为 C/C++ 写调用库
20.  使用 Rust + Wasm 为 Python 写调用库
21.  使用 Rust + Wasm 为 Ruby 写调用库
22.  使用 Rust + Wasm 为 Nodejs 写调用库
23.  使用 Rust + Wasm 为 Php 写调用库
24.  使用 Rust + Wasm 为 Go 写调用库
25.  使用 Rust + Wasm 为 Java 写调用库
26.  一些比较知名的 FFI 封装 crate
27.  后记-结语

**大纲是预定的，不排除在未来写作过程中，随时调整。**

### Rust FFI 的意义

如大纲所见，Rust 从设计之初就定位一门系统级编程语言，Rust FFI 在实现这一目标的过程中，起着特殊而重要的作用。利用 FFI，Rust 终能迅速切入巨大的存量遗产，同时也能牢牢把握新兴增量领域。或许再过几年时间，大家就能看出来 Rust 吉祥物的意义——一只螃蟹——橫着走。
