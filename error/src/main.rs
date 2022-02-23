use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn run_panic() {
    let v = vec![1, 2, 3];
    v[99];
}

fn file_open() {
    let f = File::open("hello.txt");

    let _f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("File open error!!! {:?}", error);
        }
    };
}

fn file_open2() {
    let f = File::open("hello.txt");

    let _f = match f {
        Ok(file) => file,
        // "if ***" is the "match guard" for Err condition
        // "ref" prevents being moved to guard conds
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            // create file if not exists
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!("Tried to create file, but error {:?}", e)
                }
            }
        }
        Err(error) => {
            panic!("FIle open error{:?}", error)
        }
    };
}

fn call_unwrap() {
    // panic with original message
    let _f = File::open("hello2.txt").expect("Failed to open hello2.txt");

    let _f = File::open("hello2.txt").unwrap();
}

fn propagate_error() -> Result<String, io::Error> {
    let f = File::open("hello3.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    // match f.read_to_string(&mut s){
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }

    f.read_to_string(&mut s)?;
    Ok(s)
}

//more simple style
fn propagate_error2() -> Result<String, io::Error> {
    let mut s = String::new();

    // "?" can be used only in func which returns Result
    File::open("hello4.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn main() {
    // panic!("crash and burn");

    propagate_error2().unwrap();

    propagate_error().unwrap();

    call_unwrap();

    file_open2();

    file_open();

    run_panic();
}
