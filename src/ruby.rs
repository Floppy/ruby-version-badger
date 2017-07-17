pub fn colour(version: String) -> String {
    // Check version and set colour    
    match version.as_ref() {
        "2.4.1"          => "brightgreen",
        "2.3.4"          => "yellow",
        "2.2.7"          => "orange",
        ""               => "lightgray",
        "404: Not Found" => "lightgray",
        _                => "red",
    }.to_string()
}

#[cfg(test)]
mod tests {
    use ruby;
    
    #[test]
    fn deprecated_versions_are_red() {
        assert_eq!("red", ruby::colour(String::from("1.9.3")));
    }

    #[test]
    fn current_versions_are_green() {
        assert_eq!("brightgreen", ruby::colour(String::from("2.4.1")));
    }

    #[test]
    fn supported_versions_are_yellow() {
        assert_eq!("yellow", ruby::colour(String::from("2.3.4")));
    }

    #[test]
    fn ending_versions_are_orange() {
        assert_eq!("orange", ruby::colour(String::from("2.2.7")));
    }
}
