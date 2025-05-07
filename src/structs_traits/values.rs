// traits define (like a Protocol) that a struct must implement to be part of a type.
// The trait can be used across any number of struct types; so its a little polymorphic.
pub trait SomeTrait {
    fn is_valid(&self) -> bool; // no fn body.
}
#[derive(Debug, Copy, Clone)]
pub struct RandomThings {
    pub invoke_ct: i32,
    pub random_int: i32,
    pub random_bool: bool,
}

// Add the trait to RandomThings
impl SomeTrait for RandomThings {
    // a trait fn is always pub.
    fn is_valid(&self) -> bool {
        self.random_bool
    }
}

impl Default for RandomThings {
    //the Default trait is part of the common lib, and its a trait for creating a default Self.
    fn default() -> Self {
        Self {
            invoke_ct: 0,
            random_bool: true,
            random_int: 1,
        }
    }
}

// adding a function (an associated function) to a struct
// The Self is a keyword (which is a type, hence pascal case), which is the same as saying RandomThings
impl RandomThings {
    // the new function is a function on a type (like a cls method)
    pub fn new(arg1: bool) -> Self {
        if arg1 {
            Self {
                invoke_ct: 0,
                random_bool: true,
                random_int: 1,
            } // look, no ; --- that is, return this.
        } else {
            Self {
                invoke_ct: 0,
                random_bool: false,
                random_int: 0,
            }
        }
    }

    pub fn is_smaller(&mut self, val: i32) -> bool {
        // mut the struct...
        self.invoke_ct += 1;
        // that &self is data, not the type Self.
        self.random_int < val
    }
}
