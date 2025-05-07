// Fundamentally a Memory Management idea.
// Rust has NO GARBAGE COLLECTION

#[derive(Debug, Clone)] // the clone is a macro which creates a trait impl which copies the data to a new version of self.
struct Point {
    // Structs are on the heap, even if they only have Stack vals.
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy)] // Using Clone and Copy, the borrowing works like a stack variable, and hence no need for &
struct CopyPoint {
    // Structs are on the heap, even if they only have Stack vals.
    // Some structs wont be able to implement copy, when its vars are not copyable--like a String.
    x: i32,
    y: i32,
}

pub fn play() {
    // simple example of a borrow of moved val error
    let var1 = String::from("Hello");
    let var2 = var1;
    //println!("{}", var1); // This wont compile!

    // Stack v Heap
    // Stack -- First memory create and retrieve; memory captured when var goes out-of-scope.
    // These are all on the stack.  As the memory size is known to rust at compile time.
    // They are fixed in size.  Collections (like vec) not on Stack.  Although there is a fixed sized array!
    let stack_i8: i8 = 10;
    let stack_f32: f32 = 10.0;
    let stack_bool: bool = true;
    let stack_char: char = 'a';

    if stack_i8 == 10 {
        let inside_scope: i32 = 100; // placed directly on the stack above var_char.
    } // another inner scope; then removed here.

    // HEAP
    // Memory that can grow in size (vectors, Hashmap, String, etc).
    // Memory that can live beyond the scope that created it.
    // But is cleaned up automatically when goes out-of-scope (OR...when last owner goes out-of-scope!!!!)
    let heap_vector1: Vec<i8> = Vec::new(); // the vec! macro allows to populate the vector with initial vals.
    let heap_string1: String = String::from("Hello");
    let heap_i8: Box<i8> = Box::new(10); //wait now...a HEAP i8!  The Box is a way of creating a var which would normally
                                         // reside on the stack, and instead, have it managed on the heap.  Rust creates the variable on the heap,
                                         // and then adds a reference to the variable on the stack.
    let stack_i8_2 = stack_i8;
    println!("Stack i8: {}", stack_i8); // But what is going on here....I thought there was only 1 owner of
                                        // a piece of memory!
                                        // but NO!  It is so cheap to make a copy of stack memory,
                                        // that the stack_i8_2 = stack_i8 actually copied the data!
    println!("Stack i8_2: {}", stack_i8_2);
    let heap_i8_2 = heap_i8; // This uses the same HEAP memory, but transfers ownership to heap_i8_2!
                             // heap_i8 is no longer OWNS any memory!

    //println!("Heap i8: {}", heap_i8); // compile error of borrowed here after move because ownership was transferred!
    println!("Heap i8_2: {}", heap_i8_2);

    // Ownership
    // Every piece of data has an owner
    // Only 1 owner at a time!
    // So, if we want to keep both heap_i8_2 and heap_i8, we need to borrow or clone.
    let heap_i8_3 = heap_i8_2.clone(); // remember, this is an expensive op.
    println!("Heap i8_2: {}", heap_i8_2); // so this now compiles.

    // Lets create 2 f64s, one on the stack, the other on the heap.
    let stack_f64: f64 = 10.0;
    let heap_f64: Box<f64> = Box::new(20.0);
    stack_proc(stack_f64); // Cause its on the Stack, when stack_proc is called stack_f64 is copied
                           // onto the stack to represent the proc's arg.
    println!("After Stack-Proc: {}", stack_f64); // works cause of the copy, and it hasn't been mutated after the proc!

    // Lets do the same thing with a heap val...
    // heap_proc(heap_f64); // the owner of heap_f64 gets transferred to the heap_proc parameter (arg)
    // at the end of heap_proc, the memory get cleaned up, and as it owned heap_f64 this memory is now gone!
    // ofcourse, we could change the proc to a fn and return the heap_f64 back and continue,
    // but then heap_f64 would need to be mut.

    // println!("After heap-Proc: {}", heap_f64);  // wont compile
    // This is where borrowing comes in.  We want the proc to borrow the ownership, then return the ownership after
    // the proc ends
    let heap_f64_2: Box<f64> = Box::new(20.0);
    heap_proc(&heap_f64_2); // in heap_proc, we add the & to the arg.
                            // the & says pass a reference to heap_f64_2
    println!("After heap-Proc: {}", heap_f64_2);
    // We can do the same borrowing with Stack vars, but Rust prefers to simply copy by default.

    // Back to the difference between String and str slice...
    let a_string = String::from("Hello!"); // that is a Heap var.
    let a_str_slice: &str = "There!"; // Not locked to stack or heap, but a pointer to "someone" elses location.
                                      // a str slice does not own the memory location of a slice, rather it borrows it.

    // We dont need & on the slice, as its already a borrow.  But if we use a_string after the proc we'll
    // need to let the proc borrow it.
    // This is one reason why a str slice is preferred over a String when passing to a fn.
    string_str_slice_proc(&a_string, a_str_slice);
    println!("after string_str_slice_proc: {}, {}", a_string, a_str_slice);

    // Creating multiple references to a variable (remember, there is still only 1 owner)
    let super_string = String::from("Hey there!"); // when refing super_string must be IMMUTABLE.
                                                   // which might be that it is defined without the mut keyword, or that it doesn't change between now and the use
                                                   // of ref_to_string_1/2
    let ref_to_string_1 = &super_string; // if we didn't use the &, ref_to_string_1 would now own the super_string
    let ref_to_string_2 = &super_string; // and this line wouldn't compile.

    // super_string still owns the memory location.
    println!(
        "Referencing: {}, {}, {}",
        super_string, ref_to_string_1, ref_to_string_2
    );
    // we could have declared super_string as mut and changed it here without the compiler changing.
    // or we could have changed super_string before the references.
    // This reference approach comes in handy when running calcs on tons of data on multi-cores...
    let string1 = String::from("Hey");
    let string2 = String::from("There");
    let big_data: Vec<&String> = vec![&string1, &string2];
    println!("from big data {}", a_multi_core_fn(&big_data)); // passing immutable reference
    println!(
        "The original vars: {}, {}, {:?}",
        string1, string2, big_data
    ); // which means this will compile.

    // Cloning, not borrowing, Structs
    let p1 = Point { x: 1, y: 2 };
    struct_proc(p1.clone());
    println!("After struct_proc: {:?}", p1);
    // COPYING, not borrowing, Structs
    let p2 = CopyPoint { x: 1, y: 2 };
    struct_proc(p2); // cause CopyPoint has the Copy trait, it will be automatically copied to struct_proc.
    println!("After struct_proc: {:?}", p2);
} // here is a scope exit, where the above vars are removed from memory (when the last owner goes o-o-s).

fn stack_proc(mut arg: f64) {
    arg += 0.9;
    println!("Stack Proc: {}", arg)
}

// arg is now borrowing ownership.
fn heap_proc(arg: &Box<f64>) {
    println!("Heap Proc: {}", arg)
}

fn string_str_slice_proc(s1: &String, s2: &str) {
    println!("string_str_slice_proc: {}, {}", s1, s2);
}

fn a_multi_core_fn(_vec: &Vec<&String>) -> i64 {
    // some heavy, multi-core calcs

    10
}

fn struct_proc<T>(arg: T)
where
    T: std::fmt::Debug,
{
    println!("struct_proc: {:?}", arg);
}
