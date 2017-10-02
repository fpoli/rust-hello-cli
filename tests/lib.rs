extern crate lib;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_message_none() {
        let message = lib::build_message(None);
        assert_eq!(message, "Hello World!");
    }

    #[test]
    fn build_message_some() {
        let message = lib::build_message(Some("MyName"));
        assert_eq!(message, "Hello MyName!");
    }
}
