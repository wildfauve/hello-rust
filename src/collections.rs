use std::collections::HashMap;

pub fn play() {
    vectors();

    hashmaps();
}

fn vectors() {
    // Vectors
    // Vectors allow you to store more than one value in a single data structure
    // that puts all the values next to each other in memory
    // Vecs are implemented as generics...can hold any type.
    let v1: Vec<i32> = Vec::new(); // Create an empty vector.  Note we give it a type annotation
                                   // cause Rust does not know what type we want.
    let mut v2 = vec![1, 2, 3, 4]; // Here we use the vec macro to populate the vec.  Not need for a type annotation.
                                   // mutate a vector
    v2.push(5);
    v2.push(6);
    println!("Mutated vec: {:?}", v2);

    // Getting items from a vec
    let third: &i32 = &v2[2]; // using & gives a reference to the item in the vec.
                              // will panic if the index does not exist
    println!("The third element is {third}");

    let third: Option<&i32> = v2.get(2); // get returns an Option
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // We cant have mut and immut references to the same vec.
    let mut v3 = vec![1, 2, 3, 4, 5];
    let first = &v3[0]; // immut borrow occurs here

    v3.push(6); // mut borrow occurs here, as vec items are stored next to each other, adding a new item may require
                // deallocating memory and copying it to a new loc.  In which case the ref to first
                // could now be pointing to deallocated memory!

    // println!("The first element is: {first}"); // immut borrow occurs here.

    // Looping through a vec
    for i in &v2 {
        println!("{i}");
    }
    // mut the vec in a loop
    let mut v4 = vec![100, 32, 57];
    for i in &mut v4 {
        *i += 50; // * is the deference operator; we need this to get the val (rather than the ref) so it can be changed
    }
    println!("Changed Vec: {:?}", v4)
}

fn hashmaps() {
    // All keys and vals must be the same type.
    let blu = String::from("BlueTeam");
    let gre = String::from("GreenTeam");
    let mut scores: HashMap<&String, i32> = HashMap::new();
    scores.insert(&blu, 10); // we borrowed blu here otherwise insert would take ownership.
    scores.insert(&gre, 12);
    println!("hash map of scores: {:?}", scores);
    let blu_score: Option<&i32> = scores.get(&blu); // returns an Option
    println!("Score for {} is {}", blu, blu_score.unwrap());
    let blu_score_again = scores.get(&blu).copied().unwrap_or(0); // we call copied so that the result returned is an i32
                                                                  // not a &i32

    // iterating over k,v pairs
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // For types that implement the Copy trait, like i32, the values are copied into the hash map.
    // For owned values like String, the values will be moved and the hash map will be the owner of those values.
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // now field_name and field value are invalid here, as ownership has been moved.
    // println!("{}", field_name) // so this will not compile.
    // If we insert references to vals, then the ownership is not moved to the hashmap.
    // The values that the references point to MUST BE VALID for at least as long as the hash map is valid.

    // updating a hashmap
    // Overwrite...
    scores.insert(&blu, 20);
    println!("{scores:?}");
    // Add only if key is not present
    let current_blu_score = scores.entry(&blu).or_insert(100); // entry returns an Entry Enum.
    println!("{scores:?}");
    // Update based on old value
    let splittable_str = "hello hello to all you beings out there ...";
    let mut word_map = HashMap::new();
    for word in splittable_str.split_whitespace() {
        let count = word_map.entry(word).or_insert(0); //or_insert returns a mut reference to count
        *count += 1; // to change its value we must dereference it (*)
    } // the mut ref goes out of scope here.

    println!("{word_map:?}");
}
