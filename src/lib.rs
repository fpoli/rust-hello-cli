pub fn build_message(opt_name: Option<&str>) -> String {
    let name = opt_name.unwrap_or("World");
    return format!("Hello {}!", name);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_message_none() {
        let message = build_message(None);
        assert_eq!(message, "Hello World!");
    }

    #[test]
    fn build_message_some() {
        let message = build_message(Some("MyName"));
        assert_eq!(message, "Hello MyName!");
    }
}
