// Lifetimes enforce a piece of memory is still valid for a reference
// It is about ensuring that memory doesn't get cleaned up before a reference can use it!

const SOME_INT: i32 = 10; // const have a lifetime of the entire program.
const SOME_STR_1: &str = "Hi";
const SOME_STR_2: &str = "There";

// Structs can also have lifetimes
// Again, they are only concerned with references.
// That 'b: 'a is LT sub-typing, that is, 'b lasts as least as long as 'a.
struct SomeStruct<'a, 'b: 'a> {
    a: Vec<i32>,
    // ref_b: &Vec<i32>, this wont compile.
    ref_a: &'a Vec<i32>,
    ref_b: &'b Vec<i32>,
}

pub fn play() {
    let var_a; // delay assignment

    // create a artifical inner-scope.
    {
        let var_b = String::from("Hello");
        var_a = var_b
    } // var_b memory will be cleaned up here, except that the previous line moves the mem to var_a, which will still be in scope.
    println!("We still have var_b, sort of: {}", var_a);

    // but what if I want to reference var_b in var_a

    let var_a_2: String;
    {
        let var_b = String::from("Hello");
        // var_a_2 = &var_b // create a reference to var_b
        // this will generate a compile error saying that var_b doesn't life long enough
    } // var_b removed here
      // println!("We wont have var_b anymore: {}", var_a_2); // var_a_2 would be referencing memory garbage!

    // here we know that var1 will live until the end of play()
    let var1 = 10;
    let var2: i32 = 100;
    let result1 = return_a_ref(&var1, var2); // note that var2 is not a reference, and hence lifetimes do not apply.
}

// fn return_a_ref() -> &i32 {
//    let var_a = 10;
//    &var_a
//} var_a is cleaned up here. Ref to a just wont work, because var_a has been cleaned up.

// We can explicitly define the lifetime of a val using a generic lifetime indicator ('<name>)
// In most cases the compiler know how to do this, so no need to add lifetime indicators.
// arg2 doesn't need a lifetime as its not a reference.
fn return_a_ref<'a>(arg: &'a i32, arg2: i32) -> &'a i32 {
    println!("In return a ref: {}", arg2);
    arg
} // This will work, as the scope which is providing the ref is the one which will receive the result.

// Where there is multiple ref args, the compiler gives them their own lifetime, explictly as below.
// The compiler does not need to worry about the LT of arg2 as its not returned
fn return_a_ref_2<'a, 'b>(arg: &'a i32, arg2: &'b i32) -> &'a i32 {
    println!("In return a ref: {}", arg2);
    arg
}

// But here arg2 could be returned...
fn return_a_ref_3<'a>(arg: &'a i32, arg2: &'a i32) -> &'a i32 {
    if arg > arg2 {
        arg
    } else {
        arg2 // this would generate a compile error as there is a LT mismatch!
             // Rust cant guarantee that the LT of arg2 (var2 in play()) has the same LT as arg (var1).
             // so, we make them the same LT (of 'a)
    }
}

// This is still dealing with LTs as the LT of parts of an arg (parts of the vector here) have the same
// LT as the vec itself.
fn get_vec_slice(v: &[i32]) -> &[i32] {
    &v[0..2]
}

// But here the compiler cant determine what the LT should be for the result..so we need to be specific.
// The compiler assumes 2 different LTs.  But do we need 2?  Maybe 1 will do.
fn get_vec_slice_2<'a>(v1: &'a [i32], v2: &'a [i32]) -> &'a [i32] {
    if v1.len() > v2.len() {
        &v1[0..2]
    } else {
        &v2[0..2]
    }
}

// A fn which returns a static variable
// 'static does not need to be defined in the fn <> as 'static is already reserved in the lang.
// The args can also be declared as static, which means that any arg1 or arg2 passed in MUST be a 'static (like a constant)
fn return_a_static(arg1: &'static str, arg2: &'static str) -> &'static str {
    if arg1 > arg2 {
        arg1
    } else {
        arg2
    }
}

// LTs are dealt with the same with generic fns.
// PartialOrd is a trait constaint for things can can be compared for order.
fn get_smaller<'a, T: std::cmp::PartialOrd>(arg1: &'a T, arg2: &'a T) -> &'a T {
    if arg1 < arg2 {
        arg1
    } else {
        arg2
    }
}
