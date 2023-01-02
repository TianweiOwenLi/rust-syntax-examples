
// `panic!` is for unrecoverable errors.
// actions like accessing out-of-bound array indices will cause 'panic!'.
//
// To let `panic!` abort the program and keep the profile of compiled code 
// small, out may simply put "panic = 'abort' " under "[profile.release]" in 
// the 'Cargo.toml' file.
//
// To enable backtrace, run via `RUST_BACKTRACE=1 cargo run`.
pub fn panic_syntax() {
    let a: i32 = 1;
    if a == 0 {
        panic!("Value of variable 'a' inconsistent");
    }
}


use std::fs::File;
use std::io::{Error, ErrorKind, Read};

// Suppose we want to open a file; if an error is encountered because file does 
// not exist, we shall create a new file; if we have encountered an error 
// otherwise, we shall panic. 
pub fn match_on_error() {

    // standard but tedious way
    let f_result: Result<File, Error> = File::open("temp.txt");

    let _f: File = match f_result {
        Ok(x) => x,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create("temp.txt") {
                Ok(x) => x,
                Err(e) => panic!("Unable to create file: {:?}", e),
            }, 
            other_err => {
                panic!("Unable to open file: {:?}", other_err);
            }
        }
    };

    // remark: rust seems to close a file once out of scope.

}


pub fn unwrap_expect() {
    // x.unwrap() equals y x matches to Ok(y), else panic.
    let _f: File = File::open("temp.txt").unwrap();

    // there are some other flavors of unwrap, including unwrap_or() that takes 
    // an alternative value, or unwrap_or_else() that takes a lambda expression 
    // (see future chapters).


    // x.expect() works similar to unwrap(), except that you can display a &str 
    // message if panics.
    let _f: File = File::open("temp.txt")
        .expect("temp.txt should exist");
}


// and equivalent but shorter way for match_on_error; requires lambda 
// (which will be explained in future chapters) 
pub fn short_match() {
    let _f: File = File::open("temp.txt").unwrap_or_else(|e| {
        if e.kind() == ErrorKind::NotFound {
            File::create("temp.txt").expect("Unable to create file")
        } else {
            panic!("Unable to open file: {:?}", e);
        }
    });
}


// remark: we can write a function that returns Result<T, E> if its execution 
// can potentially lead to error; in this case, whatever it returns is being 
// propagated to the caller. To make it simple, we use the '?' operator. 
pub fn err_propagation() {

    // if an error happens to occur at any one of the two question-marks, 
    // such an error will be propagated back to the caller, in the form of 
    // returning some Err(_). No need for explicit match-casing, yay!
    fn _get_str_from_file() -> Result<String, Error> {
        let mut f: File = File::open("temp.txt")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }

    // an even shorter implementation.
    //
    // Note that this also works for 'main' function: you just have to change 
    // its return type to Result<(), Box<dyn Error>>. This type of Box<T> stuff 
    // will be covered in future chapters.
    fn _shorter_get_str_from_file() -> Result<String, Error> {
        let mut s = String::new();
        File::open("temp.txt")?.read_to_string(&mut s)?;
        Ok(s)
    }

    // however this is implemented in the library, so you can just do something 
    // like `std::fs::read_to_string("temp.txt");`.


    // the '?' operator can also be used in function of return type Option<T>.
    fn _last_chr_of_fst_line(text: &str) -> Option<char> {
        return text.lines().next()?.chars().last();
    }
}
