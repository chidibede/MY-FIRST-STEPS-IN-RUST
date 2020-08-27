use std::mem;

fn main() {
    // integer types
    let a: u8 = 28;

    println!(
        "a = {} and the size is {} bytes which is {}-bits",
        a,
        mem::size_of_val(&a),
        1 * 8
    );
    // mutablity
    let mut b = 12; // The default integer type inferred by the rust compiler is 32 bits
    println!("b default = {}", b);
    b = 10;
    println!("b updated = {}", b);

    // check size of an inferred variable
    println!(
        "b = {} and the size is {} bytes which is {}-bits",
        b,
        mem::size_of_val(&b),
        4 * 8
    );

    // isize and usize
    let architecture: isize = 67;
    println!(
        "architecture = {} and the size in bytes is {} bytes which is {}-bits architecture",
        architecture,
        mem::size_of_val(&architecture),
        mem::size_of_val(&architecture) * 8
    );
    // character data type and size
    let my_char = 'x';
    println!(
        "my_char = {} and the size in bytes is {} bytes which is {}-bits",
        my_char,
        mem::size_of_val(&my_char),
        mem::size_of_val(&my_char) * 8
    );

    // float data types and sizes
    let my_float = 3.2;
    println!(
        "my_float = {} and the size in bytes is {} bytes which is {}-bits",
        my_float,
        mem::size_of_val(&my_float),
        mem::size_of_val(&my_float) * 8
    );
    

    // boolean and size
    let my_bool = true;
    println!(
        "my_bool = {} and the size in bytes is {} bytes which is {}-bits",
        my_bool,
        mem::size_of_val(&my_bool),
        mem::size_of_val(&my_bool) * 8
    );
    
}
