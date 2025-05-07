#[allow(unused_variables)]
pub fn play() {
    let str_slice: &str = "Hello"; // double quotes, immutable
                                   // reference to heap data, or on stack, or embedded into compiled code.
    let flexible_str: String = String::from("Hello"); // double quotes
                                                      // on the heap and mutable
    let string_from_slice: String = str_slice.to_string(); // convert slice into String
    let hard_coded_string: String = "Hardcoded".to_string(); // in code string slice converted to a String.
    let string_to_str_slice: &str = &string_from_slice; // & is de-referencing, doesn't copy, but is a
                                                        // ref to the original str.
    let concat_string: String = ["abc", "xyz"].concat(); // concats to str into String.
    let formatted_string: String = format!("{}{}", "abc", "xyz"); // concats to str into String.
    let added_string_str: String = hard_coded_string + str_slice; // adding a String and str, but String has to be first!
                                                                  // and all subsequent strings have to be string slices
    let added_string_str_2: String = "Hardcoded-2".to_string() + str_slice + &formatted_string; // & turns the String into a str.
    let mut mut_blank_string: String = String::new(); // new String.
    mut_blank_string.push_str(str_slice); // add a str
    mut_blank_string.push_str("hard-coded-str"); // add a hardcoded str.
    mut_blank_string.push('a'); // push a char (not a String or str)
    let substring_1: &str = &added_string_str[0..2]; // substring from inclusive, to exclusive
    let substring_2: &str = &added_string_str[0..=2]; // substring from inclusive, to inclusive
    let char_by_index: &Option<char> = &added_string_str.chars().nth(0); // a char is not just a single character, hence the nth fn.
                                                                         // Also our first option.
    match char_by_index {
        Some(ch) => println!("found {}", ch),
        None => println!("Nothing found"),
    }
    if let Some(ch) = &added_string_str.chars().nth(0) {
        println!("found {}", ch);
    }
}
