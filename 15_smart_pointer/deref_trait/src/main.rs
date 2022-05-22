// Treating Smart Pointers Like Regular References with the Deref Trait

// Following the Pointer to the Value with the Dereference Operator
fn basic() {
    let x = 5;
    let y = &x; // y is a reference pointing to the value of x.

    assert_eq!(5, x);
    assert_eq!(5, *y);  //dereference (*y is own value)
}

// Using Box<T> Like a Reference
fn using_box() {
    let x = 5;
    let y = Box::new(x); // y is an instance of a box pointing to a copied value of x

    assert_eq!(5, x);
    assert_eq!(5, *y);  //*y can be used as same.
}

// Defining Our Own Smart Pointer
#[derive(Debug)]
struct MyBox<T>(T);

impl <T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn own_smart_pointer1() {
    let x = 5;
    let _y = MyBox::new(x);

    assert_eq!(5, x);
    // assert_eq!(5, *y);
    // ERROR: error[E0614]: type `MyBox<{integer}>` cannot be dereferenced
}

// Treating a Type Like a Reference by Implementing the Deref Trait
use std::ops::Deref;
impl <T> Deref for MyBox<T> {
    type Target = T; //syntax defines an associated type for the Deref trait to use.

    fn deref(&self) -> &T {
        &self.0
    }
}

fn own_smart_pointer2() {
    let x = 5;
    let _y = MyBox::new(x);

    println!("y = {:?}", _y);
    println!("y.deref() = {:?}", _y.deref());
    println!("*(y.deref()) = {:?}", *(_y.deref()) ); // *(y.deref()) is still necessary, is the ownership system.
    println!("*y = {:?}", *_y); //Rust substitutes the * operator with a call to the deref method 

    assert_eq!(5, x);
    assert_eq!(5, *_y);
}



fn main() {
    basic();
    using_box();
    own_smart_pointer1();
    own_smart_pointer2();
}
