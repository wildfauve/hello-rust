const CONST_VAR: i32 = 100;

static mut STATIC_VAR: i32 = 100;

pub fn play() {
    // Casting.  There is no explicit casting.
    let var_i32: i32 = 10;
    let var_f32: f32 = 10.2;
    // these cant be added together.
    // let var_combined = var_f32 + var_i32;  // this wont work.  I'm looking at you JS!
    let var_combined = var_f32 as i32 + var_i32; // but this will lose data.

    // Shadowing;  Maybe don't do this.
    let var_a: i32 = 10;

    {
        // artibrary inner-scope.
        println!(
            "The inner scope can see the vars from the outer scope: {}",
            var_a
        );
        let var_a: f32 = 10.1; // now var_a is shadowed, and becomes a "new" var.
        println!("The inner scope shadowed var_a: {}", var_a);
    }
    println!("The outer scope shadowed var_a is unchanged: {}", var_a);

    // Consts
    // A const has no memory allocated.  The compiler just replaces the val of the const everywhere you use it in the code.
    println!("My constant: {}", CONST_VAR);
    // there are also rust constants; like PI...
    let pi = std::f32::consts::PI;
    println!("And PI is: {}", pi);

    // Statics
    // aka Global vars.  Rust hates them so makes it difficult to use them, it makes you wrap it in unsafe!
    unsafe {
        STATIC_VAR = 10;
    }
    // we can't even use it outside the unsafe block.
}
