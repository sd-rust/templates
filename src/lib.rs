pub fn test_me() -> &'static str{
    "Hello world!"
}

#[cfg(test)]
mod tests {

    use super::test_me;

    #[test]
    fn test() {
        assert_eq!("Hello world!", test_me());
    }
}
