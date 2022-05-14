pub fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

pub fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

use std::cmp::PartialOrd;

// Generic function with "T"
// This function can be used with Copy trait
pub fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Generic function with "T"
pub fn largest2<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest: &T = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let number_list = vec![1, 3, 5, 7];
        assert_eq!(largest_i32(&number_list), 7);
    }

    #[test]
    fn it_works2() {
        let char_list = vec!['a', 'b', 'x', 'y', 'z'];
        assert_eq!(largest_char(&char_list), 'z');
    }

    #[test]
    fn it_works3() {
        let number_list = vec![1, 3, 5, 7];
        assert_eq!(largest(&number_list), 7);
    }

    #[test]
    fn it_works4() {
        let char_list = vec!['a', 'b', 'x', 'y', 'z'];
        assert_eq!(largest(&char_list), 'z');
    }

    #[test]
    fn it_works5() {
        let number_list = vec![1, 3, 5, 7];
        assert_eq!(largest2(&number_list), &7);
    }

    #[test]
    fn it_works6() {
        let char_list = vec!['a', 'b', 'x', 'y', 'z'];
        assert_eq!(largest2(&char_list), &'z');
    }
}
