use std::fs;
use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

type MyResult<T> = std::result::Result<T, io::Error>;

pub fn play() {
    let try_file_1 = using_match("hello.txt");
    println!("We have a file: {:?}", try_file_1);

    let try_file_2 = using_nested_match("hello.txt");
    println!("We have a file: {:?}", try_file_2);

    let try_file_3 = using_closures("hello.txt");
    println!("We have a file: {:?}", try_file_3);

    let try_file_4 = using_unwrap("hello.txt");
    println!("We have a file: {:?}", try_file_4);

    let try_file_5 = using_expect("hello.txt");
    println!("We have a file: {:?}", try_file_5);

    let propogate_errors = read_username_from_file("hello.txt");
    println!(
        "We have a line from the file: '{}'",
        propogate_errors.unwrap()
    );

    // Using the ? operator...
    // ? works on anything that returns a Result or Option or another type that implements FromResidual
    let propogate_errors = read_username_from_file_using_optional_op("hello.txt");
    println!(
        "We have a line from the file: '{}'",
        propogate_errors.unwrap()
    );
    let propogate_errors_2 = read_username_from_file_using_optional_op_chained("hello.txt");
    println!(
        "We have a line from the file: '{}'",
        propogate_errors_2.unwrap()
    );
    let propogate_errors_3 = read_username_from_file_using_fs("hello.txt");
    println!(
        "We have a line from the file: '{}'",
        propogate_errors_3.unwrap()
    );
}

fn using_match(filename: &str) -> File {
    // File Open returns a Result<File, error>
    // We'll panic when the file can't be openned.
    let greeting_file_result: Result<File, std::io::Error> = File::open(filename);

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };
    greeting_file
}

fn using_nested_match(filename: &str) -> File {
    // This time we are a liitle more selective about dealing with a failure.  We use nested matches (which look a little clumsy)
    // to create the file if it doesn't exist.
    let greeting_file_result: Result<File, std::io::Error> = File::open(filename);

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(filename) {
                Ok(fc) => fc,
                Err(ex) => panic!("Problem opening the file: {ex:?}"),
            },
            other_exception => {
                panic!("Problem reading the file: {other_exception:?}")
            }
        },
    };
    greeting_file
}

fn using_closures(filename: &str) -> File {
    // Using unwrap_or_else and providing a closure (fn).
    let greeting_file = File::open(filename).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(filename).unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });
    greeting_file
}

fn using_unwrap(filename: &str) -> File {
    // Unwrap will either return the item in the result on success, or panic!
    // We dont get to choose the panic! msg.
    File::open(filename).unwrap()
}

fn using_expect(filename: &str) -> File {
    // Expect will either return the item in the result on success, or panic!
    // We can choose the panic! msg.
    File::open(filename).expect(&format!("{filename} expected to exist."))
}

// Notice that we define a custom type to describe the general result type of Result<String, io::Error>
fn read_username_from_file(filename: &str) -> MyResult<String> {
    let username_file_result = File::open(filename);

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e), // the return kw....we return early from the fn.
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_using_optional_op(filename: &str) -> Result<String, io::Error> {
    // The ? op works like the Result match op.  If the result is OK, it returns the val inside the Result.
    // If it is an Err, the Err will be returned (using the return kw) from the fn.
    // Error values that have the ? operator called on them go through the from function, defined in the
    // From trait in the standard library, which is used to convert values from one type into another.
    // When the ? operator calls the from function, the error type received is converted into the error type
    // defined in the return type of the current function.
    // Therefore we can return a custom error (which will need this from trait).
    let mut username_file_result = File::open(filename)?;
    let mut username = String::new();
    username_file_result.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_using_optional_op_chained(filename: &str) -> Result<String, io::Error> {
    let mut username = String::new();

    File::open(filename)?.read_to_string(&mut username)?; // chaining the ? op

    Ok(username)
}

fn read_username_from_file_using_fs(filename: &str) -> Result<String, io::Error> {
    // The chained ? op is so common, that it is implemented in the fs lib.
    fs::read_to_string(filename)
}
