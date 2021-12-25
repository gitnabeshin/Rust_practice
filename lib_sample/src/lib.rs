fn add(x: i32, y: i32) -> i32 {
    x + y
}

#[cfg(test)]
mod tests {
    use crate::add;

    #[test]
    fn it_works() {
        assert_eq!(3, add(1, 2));
        assert_eq!(4, add(1, 3));
    }

    #[test]
    fn it_works2() {
        assert_eq!(100, add(99, 1));
    }

}
