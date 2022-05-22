// Rc<T>, the Reference Counted Smart Pointer
// Rc: Reference counter

enum List {
    Cons(i32, Rc<List>), //Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

// Trying to implement this scenario using our definition of List with Box<T> won’t work,
fn basic() {
    // let a = Cons( 5, Box::new( Cons( 10, Box::new(Nil))));
    // let _b = Cons( 3, Box::new(a));
    // let _c = Cons( 4, Box::new(a));
}


// Cloning an Rc<T> Increases the Reference Count
use std::rc::Rc;
fn use_rc() {
    // Rc::clone doesn’t make a deep copy of all the data like most types’ implementations of clone do.
    // Rc::clone only increments the reference count, which doesn’t take much time.
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil) ))));
    println!("count after a = {}", Rc::strong_count(&a));

    let _b = Cons(3, Rc::clone(&a));
    println!("count after b = {}", Rc::strong_count(&a));

    {
        let _c = Cons(4, Rc::clone(&a));
        println!("count after c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

fn main() {
    basic();
    use_rc();
}
