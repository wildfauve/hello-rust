mod values;
use values::*;

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Thing {
    name: String,
    dob: i32,
    is_cat: bool,
    random_thing: RandomThings,
}

// That the impl is defined outside the struct helps to add implementations (extend) to someone else's struct
// which we may not have access to.
impl Thing {
    pub fn is_older(&self, year: i32) -> bool {
        // that &self is data, not the type Self.
        self.dob <= year
    }
}
impl SomeTrait for Thing {
    fn is_valid(&self) -> bool {
        true
    }
}

// Now we can create a more generate fn which deals with types with a specific trait
// We havent done the dyn kw yet!
fn show_if_valid(me: &dyn SomeTrait) {
    if me.is_valid() {
        println!("It is valid");
    }
}
#[allow(unused_variables)]
pub fn play() {
    let dinsdale = Thing {
        // this will be immutable
        name: "Dinsdale".to_string(),
        dob: 1991,
        is_cat: true,
        random_thing: RandomThings::new(true),
    };
    let mut larry = Thing {
        // this will be mutable
        name: "Larry".to_string(),
        dob: 2023,
        is_cat: true,
        random_thing: RandomThings::new(false),
    };
    larry.dob = 2024;

    // set new vars from existing struct
    let gavin = Thing {
        name: "Gavin".to_string(),
        ..larry
    };

    // visability of mod structs
    // As RandomThings is in a different file, the struct must be public as well as the fields.
    let mut my_random = RandomThings {
        invoke_ct: 0,
        random_int: 1,
        random_bool: true,
    };

    let is_smaller = my_random.is_smaller(10);
    let is_valid = my_random.is_valid();

    let is_older = dinsdale.is_older(2020);
    // using the polymorphic fn with 2 different structs.
    show_if_valid(&dinsdale);
    show_if_valid(&my_random);

    // using the default trait
    let my_default_random = RandomThings::default();

    // remember that "{:?}"  in tuples.rs.  Well this is the debug trait.  That is, to print a type/value the println
    // needs to know how to print it.  Now, we could implement the trait fn debug in the type.  But this is
    // so common, that there is a macro as a compiler annotation [derive(Debug)] which is applied to the
    // struct.  So now we can...
    println!("My Struct looks like this {:?}", my_default_random);
}
