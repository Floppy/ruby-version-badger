pub fn colour(version: String) -> String {
    // Check version and set colour    
    match version.as_ref() {
        "1.20.0"         => "brightgreen",
        ""               => "lightgray",
        "404: Not Found" => "lightgray",
        _                => "red",
    }.to_string()
}

#[cfg(test)]
mod tests {
    use rust;
    
    #[test]
    fn deprecated_versions_are_red() {
        assert_eq!("red", rust::colour(String::from("1.18.0")));
    }

    #[test]
    fn current_versions_are_green() {
        assert_eq!("brightgreen", rust::colour(String::from("1.20.0")));
    }
}
