/*******************************************************
 * 13.2. Processing a Series of Items with Iterators
 *******************************************************/

#[allow(dead_code)]
fn test1() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }
}

// PartialEq: compare objects
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

// return only shoes that are the specified size.
#[allow(dead_code)]
fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
    // The filter method on an iterator takes a closure that takes each item
    // from the iterator and returns a Boolean. If the closure returns true,
    // the value will be included in the iterator produced by filter.
}

// implement the Iterator trait for our Counter type by defining the body of the next method
struct Counter {
    count: u32,
}

impl Counter {
    #[allow(dead_code)]
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

// Iterator trait
// pub trait Iterator {
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;
//     // methods with default implementations
// }
impl Iterator for Counter {
    type Item = u32; // make this type same as Counter::count type

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {

    use super::test1;

    #[test]
    fn it_works() {
        test1();
    }

    // The Iterator Trait and the next Method
    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];
        let mut v1_iter = v1.iter();

        // consuming adaptors
        // .next() will change v1_iter's internal state.
        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    // Methods that Consume the Iterator
    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();

        // sum() moves iter ownership
        let total: i32 = v1_iter.sum();
        assert_eq!(total, 6);

        // ERROR: v1_iter is already moved.
        // assert_eq!(v1_iter.next(), None);
    }

    // Methods that Produce Other Iterators
    #[test]
    fn iterator_adapter() {
        let v1: Vec<i32> = vec![1, 2, 3];

        // map() can use closure, and collect() for another Vector.
        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
        assert_eq!(v2, vec![2, 3, 4]);
    }

    // Using Closures that Capture Their Environment
    use crate::shoes_in_my_size;
    use crate::Shoe;

    #[test]
    fn filers_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_my_size(shoes, 10);

        // Shoe must have #[derive(Debug,PartialEq)] for object comparison
        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }

    // Creating Our Own Iterators with the Iterator Trait
    use crate::Counter;

    #[test]
    fn calling_next_directly() {
        let mut counter = Counter::new();

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn iterator_adapters() {
        let a1 = vec![1, 2, 3];
        let a2 = vec![4, 5, 6];
        let mut iter = a1.iter().zip(a2.iter());
        assert_eq!(iter.next(), Some((&1, &4)));
        assert_eq!(iter.next(), Some((&2, &5)));
        assert_eq!(iter.next(), Some((&3, &6)));
        assert_eq!(iter.next(), None);

        let a = vec![1, 2, 3];
        let mut iter = a.iter().map(|x| x * 2);
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next(), Some(6));
        assert_eq!(iter.next(), None);

        let a = vec![0i32, 1, 2];
        let mut iter = a.iter().filter(|x| x.is_positive());
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), None);

        let a = vec![1, 2, 3];
        let sum = a.iter().fold(0, |acc, x| acc + x);
        assert_eq!(sum, 6);
        let sum: i32 = a.iter().sum();
        assert_eq!(sum, 6);

        let a = vec![1, 2, 3];
        let doubled: Vec<i32> = a.iter().map(|x| x * 2).collect();
        assert_eq!(doubled, vec![2, 4, 6]);

        let a = vec!['a', 'b', 'c'];
        let mut iter = a.iter().enumerate();
        assert_eq!(iter.next(), Some((0, &'a')));
        assert_eq!(iter.next(), Some((1, &'b')));
        assert_eq!(iter.next(), Some((2, &'c')));
        assert_eq!(iter.next(), None);

        // inspect() return iterator
        let a = vec![1, 2, 3];
        let sum: i32 = a
            .iter()
            .clone()
            .inspect(|x| println!("in filter: {}", x))
            .sum();
        assert_eq!(sum, 6);
    }

    // Using Other Iterator Trait Methods
    #[test]
    fn using_other_iterator_trait_methods() {
        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();

        assert_eq!(sum, 18);
    }
}
