mod util;

fn foo() ->   String {
    1.to_string()
}

#[cfg(test)]
mod tests {

    #[test]
    fn assert_works() {
        assert!(1 == 1, "it totally did't work")
    }
    #[test]
    fn assert_eq_works() {
        assert_eq!(2 + 2, 4);
    }
}
