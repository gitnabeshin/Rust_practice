extern crate cc;

fn main() {
    cc::Build::new()
        .file("lib_src/foo.c")
        .include("src")
        .compile("libfoo.a");
}
