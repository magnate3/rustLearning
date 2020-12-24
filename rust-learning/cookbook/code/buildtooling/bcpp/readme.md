# build tooling

[](https://rust-lang-nursery.github.io/rust-cookbook/development_tools/build_tools.html#compile-and-link-statically-to-a-bundled-c-library-1)

## Compile and link statically to a bundled C++ library

[![cc-badge](https://badge-cache.kominick.com/crates/v/cc.svg?label=cc)](https://docs.rs/cc)  [![cat-development-tools-badge](https://badge-cache.kominick.com/badge/development_tools--x.svg?style=social)](https://crates.io/categories/development-tools)

Linking a bundled C++ library is very similar to linking a bundled C library. The two core differences when compiling and statically linking a bundled C++ library are specifying a C++ compiler via the builder method  [`cpp(true)`](https://docs.rs/cc/*/cc/struct.Build.html#method.cpp)  and preventing name mangling by the C++ compiler by adding the  `extern "C"`  section at the top of our C++ source file.
