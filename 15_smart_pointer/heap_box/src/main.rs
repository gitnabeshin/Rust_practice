// Using Box<T> to Point to Data on the Heap

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil, //None,
}

use List::{Cons, Nil};

fn main() {
    // b is allocaterd on the heap
    let b = Box::new(5);
    println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    println!("list {:?}", list);
}
