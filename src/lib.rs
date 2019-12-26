mod util;

#[cfg(test)]
mod tests {

    #[test]
    fn assert_works() {
        assert!(1 == 2, "it totally did not work")
    }
    #[test]
    fn assert_eq_works() {
        assert_eq!(2 + 2, 5);
    }
}
