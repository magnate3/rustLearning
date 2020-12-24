
[](https://rust-lang-nursery.github.io/rust-cookbook/development_tools/build_tools.html#build-time-tooling)

# Build Time Tooling

This section covers "build-time" tooling, or code that is run prior to compiling a crate's source code. Conventionally, build-time code lives in a  **build.rs**  file and is commonly referred to as a "build script". Common use cases include rust code generation and compilation of bundled C/C++/asm code. See crates.io's  [documentation on the matter](http://doc.crates.io/build-script.html)  for more information.

[](https://rust-lang-nursery.github.io/rust-cookbook/development_tools/build_tools.html#compile-and-link-statically-to-a-bundled-c-library)

## Compile and link statically to a bundled C library

[![cc-badge](https://badge-cache.kominick.com/crates/v/cc.svg?label=cc)](https://docs.rs/cc)  [![cat-development-tools-badge](https://badge-cache.kominick.com/badge/development_tools--x.svg?style=social)](https://crates.io/categories/development-tools)

To accommodate scenarios where additional C, C++, or assembly is required in a project, the  [**cc**](https://docs.rs/cc)  crate offers a simple api for compiling bundled C/C++/asm code into static libraries (**.a**) that can be statically linked to by  **rustc**.

The following example has some bundled C code (**src/hello.c**) that will be used from rust. Before compiling rust source code, the "build" file (**build.rs**) specified in  **Cargo.toml**  runs. Using the  [**cc**](https://docs.rs/cc)  crate, a static library file will be produced (in this case,  **libhello.a**, see  [`compile`  docs](https://docs.rs/cc/*/cc/struct.Build.html#method.compile)) which can then be used from rust by declaring the external function signatures in an  `extern`  block.

Since the bundled C is very simple, only a single source file needs to be passed to  [`cc::Build`](https://docs.rs/cc/*/cc/struct.Build.html). For more complex build requirements,  [`cc::Build`](https://docs.rs/cc/*/cc/struct.Build.html)  offers a full suite of builder methods for specifying  [`include`](https://docs.rs/cc/*/cc/struct.Build.html#method.include)  paths and extra compiler  [`flag`](https://docs.rs/cc/*/cc/struct.Build.html#method.flag)s.

[](https://rust-lang-nursery.github.io/rust-cookbook/development_tools/build_tools.html#cargotoml)

### `Cargo.toml`

`[package]
...
build = "build.rs"
 [build-dependencies]
cc = "1"
 [dependencies]
error-chain = "0.11"` 

[](https://rust-lang-nursery.github.io/rust-cookbook/development_tools/build_tools.html#buildrs)

### `build.rs`

``extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/hello.c")
        .compile("hello");   // outputs `libhello.a`
}`` 

[](https://rust-lang-nursery.github.io/rust-cookbook/development_tools/build_tools.html#srchelloc)

### `src/hello.c`

`#include <stdio.h>

void hello() {
    printf("Hello from C!\n");
}

void greet(const char* name) {
    printf("Hello, %s!\n", name);
}` 

[](https://rust-lang-nursery.github.io/rust-cookbook/development_tools/build_tools.html#srcmainrs)

### `src/main.rs`

`#[macro_use] extern crate error_chain; use std::ffi::CString;
use std::os::raw::c_char;   error_chain! { foreign_links { NulError(::std::ffi::NulError); Io(::std::io::Error); } }   fn prompt(s: &str) -> Result<String> { use std::io::Write; print!("{}", s); std::io::stdout().flush()?; let mut input = String::new(); std::io::stdin().read_line(&mut input)?; Ok(input.trim().to_string()) }

extern {
    fn hello();
    fn greet(name: *const c_char);
}

fn main() -> Result<()> {
    unsafe { hello() }
    let name = prompt("What's your name? ")?;
    let c_name = CString::new(name)?;
    unsafe { greet(c_name.as_ptr()) }
    Ok(())
}` 

[](https://rust-lang-nursery.github.io/rust-cookbook/development_tools/build_tools.html#compile-and-link-statically-to-a-bundled-c-library-1)

## Compile and link statically to a bundled C++ library

[![cc-badge](https://badge-cache.kominick.com/crates/v/cc.svg?label=cc)](https://docs.rs/cc)  [![cat-development-tools-badge](https://badge-cache.kominick.com/badge/development_tools--x.svg?style=social)](https://crates.io/categories/development-tools)

Linking a bundled C++ library is very similar to linking a bundled C library. The two core differences when compiling and statically linking a bundled C++ library are specifying a C++ compiler via the builder method  [`cpp(true)`](https://docs.rs/cc/*/cc/struct.Build.html#method.cpp)  and preventing name mangling by the C++ compiler by adding the  `extern "C"`  section at the top of our C++ source file.

[](https://rust-lang-nursery.github.io/rust-cookbook/development_tools/build_tools.html#cargotoml-1)

### `Cargo.toml`

`[package]
...
build = "build.rs"
 [build-dependencies]
cc = "1"` 

[](https://rust-lang-nursery.github.io/rust-cookbook/development_tools/build_tools.html#buildrs-1)

### `build.rs`

`extern crate cc;

fn main() {
    cc::Build::new()
        .cpp(true)
        .file("src/foo.cpp")
        .compile("foo");   
}` 

[](https://rust-lang-nursery.github.io/rust-cookbook/development_tools/build_tools.html#srcfoocpp)

### `src/foo.cpp`

`extern "C" {
    int multiply(int x, int y);
}

int multiply(int x, int y) {
    return x*y;
}` 

[](https://rust-lang-nursery.github.io/rust-cookbook/development_tools/build_tools.html#srcmainrs-1)

### `src/main.rs`

`extern {
    fn multiply(x : i32, y : i32) -> i32;
}

fn main(){
    unsafe {
        println!("{}", multiply(5,7));
    }   
}` 

[](https://rust-lang-nursery.github.io/rust-cookbook/development_tools/build_tools.html#compile-a-c-library-while-setting-custom-defines)

## Compile a C library while setting custom defines

[![cc-badge](https://badge-cache.kominick.com/crates/v/cc.svg?label=cc)](https://docs.rs/cc)  [![cat-development-tools-badge](https://badge-cache.kominick.com/badge/development_tools--x.svg?style=social)](https://crates.io/categories/development-tools)

It is simple to build bundled C code with custom defines using  [`cc::Build::define`](https://docs.rs/cc/*/cc/struct.Build.html#method.define). The method takes an  [`Option`](https://doc.rust-lang.org/std/option/enum.Option.html)  value, so it is possible to create defines such as  `#define APP_NAME "foo"`  as well as  `#define WELCOME`  (pass  `None`  as the value for a value-less define). This example builds a bundled C file with dynamic defines set in  `build.rs`  and prints "**Welcome to foo - version 1.0.2**" when run. Cargo sets some  [environment variables](https://doc.rust-lang.org/cargo/reference/environment-variables.html)  which may be useful for some custom defines.

[](https://rust-lang-nursery.github.io/rust-cookbook/development_tools/build_tools.html#cargotoml-2)

### `Cargo.toml`

`[package]
...
version = "1.0.2"
build = "build.rs"
 [build-dependencies]
cc = "1"` 

[](https://rust-lang-nursery.github.io/rust-cookbook/development_tools/build_tools.html#buildrs-2)

### `build.rs`

`extern crate cc;

fn main() {
    cc::Build::new()
        .define("APP_NAME", "\"foo\"")
        .define("VERSION", format!("\"{}\"", env!("CARGO_PKG_VERSION")).as_str())
        .define("WELCOME", None)
        .file("src/foo.c")
        .compile("foo");
}` 

[](https://rust-lang-nursery.github.io/rust-cookbook/development_tools/build_tools.html#srcfooc)

### `src/foo.c`

`#include <stdio.h>

void print_app_info() {
#ifdef WELCOME
    printf("Welcome to ");
#endif
    printf("%s - version %s\n", APP_NAME, VERSION);
}` 

[](https://rust-lang-nursery.github.io/rust-cookbook/development_tools/build_tools.html#srcmainrs-2)

### `src/main.rs`

`extern {
    fn print_app_info();
}

fn main(){
    unsafe {
        print_app_info();
    }   
}`
