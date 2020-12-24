# Compile and link statically to a bundled C library

To accommodate scenarios where additional C, C++, or assembly is required in a project, the cc crate offers a simple api for compiling bundled C/C++/asm code into static libraries (.a) that can be statically linked to by rustc.

The following example has some bundled C code (src/hello.c) that will be used from rust. Before compiling rust source code, the "build" file (build.rs) specified in Cargo.toml runs. Using the cc crate, a static library file will be produced (in this case, libhello.a, see compile docs) which can then be used from rust by declaring the external function signatures in an extern block.

Since the bundled C is very simple, only a single source file needs to be passed to cc::Build. For more complex build requirements, cc::Build offers a full suite of builder methods for specifying include paths and extra compiler flags.
