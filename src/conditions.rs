pub fn play() {
    // conditions
    let test1 = true;
    let test2 = 30;
    let test3 = 50;
    if test1 || test2 > 100 && test3 == 200 {
        println!("Test passes");
    } else if !test1 {
        println!("Test not test1");
    } else {
        println!("Test any other");
    }
    // note that the cond is like a mini fn, so, when returning 100, it doesn't have a ;
    let var_from_inline = if test1 { 100 } else { 200 };

    match test2 {
        0 => {
            println!("Here1")
        }
        1 | 2 => {
            println!("Here1.1")
        }
        1..=100 => {
            println!("Here2")
        }
        _ => {
            println!("Here3")
        }
    }
    let some_inline_match = match test2 {
        0 => 100,
        _ => 0,
    };
}
