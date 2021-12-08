/*
Primitive types:
Integers: u8, i8, u16, i16, u32, i32, u64, i64, i128, u128 (i => signed u => unsigned)
(number corresponds to number of bits ==> of memory)
Floats: f32, f64
Boolean (bool)
Character (char)

Tuples
Arrays
*/


pub fn run() {
    //Find max size
    println!("Max i32: {}", i32::MAX);
    println!("Max i64: {}", i64::MAX);

    //Boolean
    let is_active:bool = true;
    //Get boolean from expression
    let is_greater:bool = 10 < 1;

    // Char
    let a1:char = 'a';
    let face = '\u{1F600}';

    println!("{:?}", (is_active, is_greater, a1, face));


}