use std::{thread, time::Duration};

#[derive(Debug, PartialEq, Clone, Copy)]
enum ShirtColour {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColour>,
}

impl Inventory {
    fn giveaway(&self, user_pref: Option<ShirtColour>) -> ShirtColour {
        // unwrap_or_else takes a closure which has no arguments that returns a value <T> which is the same type
        // stored in the Some variant of Option<T>; in this case a <ShirtColour>
        // It closes around the self<Inventory> instance.  This is not something we can do with a fn.
        user_pref.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColour {
        let mut num_red = 0;
        let mut num_blue = 0;
        for colour in &self.shirts {
            match colour {
                ShirtColour::Red => num_red += 1,
                ShirtColour::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColour::Red
        } else {
            ShirtColour::Blue
        }
    }
}

pub fn play() {
    let store_inv = Inventory {
        shirts: vec![ShirtColour::Blue, ShirtColour::Blue, ShirtColour::Red],
    };

    let usr_pref_1 = Some(ShirtColour::Red);
    let giveaway1 = store_inv.giveaway(usr_pref_1);
    println!("User with ref {:?} gets {:?}", usr_pref_1, giveaway1);

    let usr_pref_2 = None;
    let giveaway2 = store_inv.giveaway(usr_pref_2);
    println!("User with ref {:?} gets {:?}", usr_pref_2, giveaway2);

    // Closures do not actually need explict arg types; but they can be optionally added/
    let long_running_closure = |num: u32| -> u32 {
        println!("Lets wait for a sec");
        thread::sleep(Duration::from_secs(2));
        num
    };

    // versions of closure syntax (compared with a fn)
    fn add_one_v1(x: u32) -> u32 {
        x + 1
    }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x: u32| x + 1;

    // Closures can capture values from their environment in three ways, which directly map to the three ways a function
    // can take a parameter: borrowing immutably, borrowing mutably, and taking ownership.
    //                       -------------------  -----------------      ----------------
    // 1.  Immutable Borrow
    let list_immut = vec![1, 2, 3];
    println!("Immutable Borrow: Before defining closure: {list_immut:?}");

    let only_borrows = || println!("From closure: {list_immut:?}");

    println!("Before calling closure: {list_immut:?}");
    only_borrows();
    println!("After calling closure: {list_immut:?}");

    // 2. Mutable Borrow
    let mut list_mut = vec![1, 2, 3];
    println!("Mutable Borrow: Before defining closure: {list_mut:?}");

    let mut borrows_mutably = || list_mut.push(7); // captures a mut reference to list_mut
    // println!("Before calling closure: {list_mut:?}");  // This wont compile an immutable borrow to print isn’t allowed because no other borrows are allowed when there’s a mutable borrow
    borrows_mutably();
    println!("After calling closure: {list_mut:?}");

    // We can also move the var into the closure.  We might do this if we were going to run the closure in another thread.
    // The move is a kw.
    let list_2 = vec![1, 2, 3];
    println!("Before defining closure: {list_2:?}");

    thread::spawn(move || println!("From thread: {list_2:?}"))
        .join()
        .unwrap();

    // The way a closure captures and handles values from the environment affects which traits the closure implements,
    // and traits are how functions and structs can specify what kinds of closures they can use.
    // 1. FnOnce.   Closures that can be called once. All closures implement at least this trait, because all closures can be called.
    //              A closure that moves captured values out of its body will only implement FnOnce and none of the other Fn traits, because it can only be called once.
    // 2. FnMut.    Closures that don’t move captured values out of their body, but that might mutate the captured values.
    //              These closures can be called more than once.
    // 3. Fn.       Closures that don’t move captured values out of their body and that don’t mutate captured values,
    //              as well as closures that capture nothing from their environment.
    //              These closures can be called more than once without mutating their environment, which is important in cases such as calling a closure multiple times concurrently.
}
