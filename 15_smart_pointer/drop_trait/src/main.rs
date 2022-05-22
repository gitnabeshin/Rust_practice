// Running Code on Cleanup with the Drop Trait

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("  Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer { data: String::from("my stuff") };
    let _d = CustomSmartPointer { data: String::from("other stuff") };
    println!("CustomSmartPointers created.");

    // explicit destructor calls not allowed
    // _c.drop();
    // println!("CustomSmartPointer dropped before the end of main.");

    drop(c);
    println!(" **CustomSmartPointer dropped before the end of main.");

}
