#[link(name = "foo", kind = "static")]
extern "C" {
    fn func_a(input: i32) -> i32;
    fn func_b(input: i32) -> *const i8;
}

fn main() {
    use std::ffi::CStr;

    unsafe {
        println!("Rust: ret from C func_a: {}", func_a(1));
    }
    let str_ = unsafe { CStr::from_ptr(func_b(10)) }.to_str().unwrap();
    println!("Rust: ret from C func_b: {}", str_);
}
