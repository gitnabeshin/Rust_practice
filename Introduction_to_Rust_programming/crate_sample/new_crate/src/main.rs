use new_crate::module_a;
use new_crate::module_b;

fn main(){
    println!("{}", "main start...");
    module_a::func();
    println!("{}", "call from main-----");
    module_b::module_c::func();
    module_b::module_d::func();
}