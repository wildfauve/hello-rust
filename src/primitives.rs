#[allow(unused_variables, unused_mut)]
pub fn play() {
    let boolean = true; // immutable
    let mut mut_bool = true; // mutable

    let signed_small_int: i8 = 10; // int with 8 bits (-128->+127) std::i8::MIN == -127
    let unsigned_small_int: u8 = 10; // upto +127
    let untyped_int = 10; // assumes i32

    let signed_float: f32 = 10.0; // assumes f64

    let four_byte_char: char = 'a'; // 4 bytes; not a string (which is double quotes)
}
