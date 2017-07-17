pub fn colour(version: String) -> String {
    // Check version and set colour
    println!("version being checked: '{}'", version);
    let mut colour = "red";
    if version == "2.4.1" {
        // current
        colour = "brightgreen";
    }
    else if version == "2.3.4" {
        // previous but in lifetime
        colour = "yellow";
    }
    else if version == "2.2.7" {
        // approaching EOL
        colour = "orange";
    }
    else if version == "" || version == "404: Not Found" {
        // unknown
        colour = "lightgray";
    }
    return String::from(colour);
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
