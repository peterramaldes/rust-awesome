use std::fs::File;
use std::io::ErrorKind;

fn main() {
    another_way_to_handling_with_unwrap_and_else();
}

/// This is one way to handling error, using match
fn one_way_to_handling_with_match() {
    let result = File::open("hello.txt");
    let file = match result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}

/// This is another way to handling errors, using unwrap or else
fn another_way_to_handling_with_unwrap_and_else() {
    let file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem to creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}
