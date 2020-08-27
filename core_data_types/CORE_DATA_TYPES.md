# Rust Note

### RUST EXPLAINED
Rust is a systems programming language that is fast and safe with a great memoory management

### Cargo
Cargo is the the package manager for rust programming. Cargo is used to download dependencies and compile packages and makes distributed packages.

### Setting up a rust project
After installation, choose a directory you want to set up your rust project and type
```cargo new <Project Name>```
The structure is as follows, a src directory and a target directory. A src directory contains the main.rs file, this file is the main file that is run by the rust program.

### Integer Types
There are different integer primitives in the rustlang. They include signed and unsigned integers (i8, i16, i32, i64, i128, isize) and the unsigned types (u8, u16, u32, u64, u128 and usize).

the isize and usize talks about size of the system architecture

##### integer data type values
signed integer data types ranges from -[2^(n-1)] to [2^(n-1) - 1]
unsigned integer data types ranges from 0 to [(2^n) - 1]

If you don't specify any integer, the rust compiler defaults to i32. A good point to note is that if you
have a value that is greater than the type assigned, rustc -the rust compiler will panic with an integer overflow at runtime. This helps inform you that the value you are assigning is greater than the data type. This happens at runtime.

Rust can also handle integer literals like hex, decimals, octal and binary.

#### Mutability
By default, rust variables are immutable (cannot be changed). if you want to change it you must add the mut keyword
```
let mut a = 20;
a = 45;
```
The above code will work because the mut word was added.

