pub fn build_message(opt_name: Option<&str>) -> String {
    let name = opt_name.unwrap_or("World");
    return format!("Hello {}!", name);
}
