#[allow(unused_variables)]
pub fn some_procedure(arg1: f32, arg2: i128) {
    // procedures dont return anything.
    println!("I'm in some_procedure");
}

pub fn print_string(arg1: String) {
    println!("I have a real String {}", arg1)
}

pub fn some_fn(arg1: f32, arg2: i128) -> f32 {
    println!("I'm in some_fn");
    let val = arg1 * arg2 as f32;
    val // no ; means this is what is returned.
}
