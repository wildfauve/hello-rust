#[allow(unused_variables)]
pub fn play() {
    let some_tuple = (1, 1.1, (10, 100)); // with a nested tuple
    println!("The Tuple Parts {} {}", some_tuple.0, some_tuple.1);
    println!("The Entire Tuple {:?}", some_tuple); // :? something about console printing--of structured data types perhaps

    let from_nested_tuple = (some_tuple.2).0; // weird syntax to get to nested tuple item.

    // tuple exploding
    let (red, green, blue) = rgb();

    // The unit (or, empty) tuple
    let empty_tuple = (); // interestingly returned from a procedure.
}

fn rgb() -> (u8, u8, u8) {
    (100, 10, 50)
}
