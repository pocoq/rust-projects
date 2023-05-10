use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

fn main() {
    // raise_panic();
    // raise_errors();

    match last_char_of_first_line("\n hello") {
        Some(result) => println!("{result}"),
        None => println!("No text"),
    };
}

// with unrecoverable errors
fn raise_panic() {
    // panic!("crash and burn");

    let v = vec![1, 2, 3];
    v[99];
}

// with recoverable errors
fn raise_errors() {
    // let greeting_file_result = File::open("hello.txt");

    // ** Throw panic if cannot open/have file **
    // let greeting_file = match greeting_file_result {
    // 	Ok(file) => file,
    // 	Err(error) => panic!("Problem opening the file: {:?}", error),
    // };

    // ** Use match **
    // let greeting_file = match greeting_file_result {
    // 	Ok(file) => file,
    // 	Err(error) => match error.kind(){
    // 		ErrorKind::NotFound => match File::create("hello.txt") {
    // 			Ok(fc) => fc,
    // 			Err(e) => panic!("Problem creating file:{:?}", e),
    // 		},
    // 		other_error => {
    // 			panic!("Problem opening file: {:?}", other_error)
    // 		},
    // 	},
    // };

    // ** Use closure & unwrap or else**
    // let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem creating the file: {:?}", error);
    //         })
    //     } else {
    //         panic!("Problem opening the file: {:?}", error);
    //     }
    // });

    // ** Use expect
    // let hello_file = File::open("hello1.txt").unwrap();
    // let hello2_file =
    //     File::open("hello2.txt").expect("hello2.txt should be included in this project");

    let result = read_username_from_file();
    println!("{:?}", result);
}

// Propagating errors
fn read_username_from_file() -> Result<String, io::Error> {
    // let mut username = String::new();

    // the old way to implement propagating errors
    // let username_file_result = File::open("hello.txt");
    // let mut username_file = match username_file_result{
    // 	Ok(file) => file,
    // 	Err(e) => return Err(e),
    // };
    // match username_file.read_to_string(&mut username) {
    // 	Ok(_) => Ok(username),
    // 	Err(e) => Err(e),
    // }

    // the ? operators
    // File::open("hello.txt")?.read_to_string(&mut username)?;
    // Ok(username)

    // Shorter way
    fs::read_to_string("hello.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}


