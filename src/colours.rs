use config;

pub fn for_version(language: &String, version: &String) -> String {
    // Check version and set colour
    let versions = config::language(language.to_string()).unwrap();
    let latest = versions["latest"].as_vec().unwrap();
    let mut colour = "red";
    for v in latest {
        if v.as_str().unwrap() == version {
            colour = "green";
        }
    }
    // Everything else is red
    return colour.to_string();
}

#[cfg(test)]
mod tests {
    use colours;

    #[test]
    fn deprecated_versions_are_red() {
        assert_eq!("red", colours::for_version(
            &"test".to_string(),
            &"1.0.0".to_string())
        );
    }

    #[test]
    fn latest_versions_are_green() {
        assert_eq!("green", colours::for_version(
            &"test".to_string(),
            &"2.3.4".to_string(),
        ));
    }

    #[test]
    fn supported_versions_are_yellow() {
        assert_eq!("yellow", colours::for_version(
            &"test".to_string(),
            &"1.2.3".to_string(),
        ));
    }

    #[test]
    fn maintenance_versions_are_orange() {
        assert_eq!("orange", colours::for_version(
            &"test".to_string(),
            &"1.0.3".to_string(),
        ));
    }

    #[test]
    fn blank_strings_are_grey() {
        assert_eq!("lightgray", colours::for_version(
            &"test".to_string(),
            &"".to_string(),
        ));
    }

    #[test]
    fn not_found_is_grey() {
        assert_eq!("lightgray", colours::for_version(
            &"test".to_string(),
            &"404: Not Found".to_string(),
        ));
    }

}
