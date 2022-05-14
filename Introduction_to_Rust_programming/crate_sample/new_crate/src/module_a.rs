use crate::module_b;

mod module_a {
}

pub fn func() {
    println!("{}", "mod_a func.");
    println!("{}", "call from mod_a-----");
    module_b::module_c::func();
    module_b::module_d::func();
}
