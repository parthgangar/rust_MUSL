# rust_MUSL

Steps of how to run the rust implementation of MUSL:

The Rust module is compiled as a dynamic library, by setting the [lib] tag in the Cargo.toml file
[lib]
crate-type =["cdylib"]

All Rust functions should be marked as #[no_mangle] and extern because they will be marked as exported in the resulting library

A mere cargo build will compile lib.rs file and place the result in target/debug subdirectory of the Rust project directory. My lib is called libproject2.so

C function is compiled using 
$ gcc -g call_rust.c -o call_rust -lproject2 -L./project2/target/debug

    -g is for including symbols for debugging with gdb
    -o call_rust will create call_rust as the executable file
    -lrustcalls tells the compiler to look for function definitions in the librustcalls.so shared lib file
    -L./rustcalls/target/debug tells the compiler to look into this directory for the previous file

call_rust.c is the C file

It's necessary to tell the linker where to find this file, using the LD_LIBRARY_PATH environment variable
$ LD_LIBRARY_PATH=project2/target/debug ldd call_rust

Now to execute
$ LD_LIBRARY_PATH=project2/target/debug ./call_rust 

