pub fn vector_io() {
    let mut v1: Vec<i32> = Vec::new();
    v1.push(1);
    v1.push(2);
    v1.push(3);

    let v2 = vec![4, 5, 6];

    // this will be paniced.
    // let will_panic = &v2[100];
    // let will_panic = v2.get(100);

    // get() with index out of bounds will return None.
    let index = 100;
    match v2.get(index) {
        Some(i) => println!("v2[{}] = {}. exist.", index, i),
        None => println!("v2[{}]: not exist.", index),
    }

    println!("v1: {:?}", v1);
    println!("v2: {:?}", v2);
}

pub fn vector_borrow() {
    let mut v = vec![1, 2, 3, 4, 5];

    // mutable borrow after imutable is not allowed.

    // let first = &v[0]; // imutable borrow
    let first = v[0];

    // mutable borrow
    v.push(6);

    println!("The first element is : {}", first);
}

pub fn vector_access() {
    let mut v = vec![1, 2, 3, 4];
    println!("--- vector access ---");
    for i in &v {
        println!("{}", i);
    }

    for i in &mut v {
        *i *= 100;
    }
    println!("{:?}", v);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        vector_io();
    }

    #[test]
    fn it_works2() {
        vector_borrow();
    }

    #[test]
    fn it_works3() {
        vector_access();
    }
}
