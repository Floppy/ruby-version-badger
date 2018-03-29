use regex::Regex;
use github;
use reqwest;

pub fn detected(user: &String, repo: &String) -> Result<bool, reqwest::Error> {
    return github::exists(
        user, 
        repo, 
        &"master".to_string(), 
        &"Cargo.toml".to_string()
    );
}

pub fn name() -> String {
    return "rust".to_string();
}

pub fn version(user: &String, repo: &String) -> Result<String, reqwest::Error> {
    let mut version = "unknown".to_string();
    // Get ruby version from Gemfile
    let file = github::get(user, repo, &"master".to_string(), &"Cargo.toml".to_string());
    version = version_from_cargo(file.unwrap());
    debug!("version from Cargo.toml: '{}'", version);    
    // fall back to rust-toolchain
    if version == "" {
        // Get a file
        version = github::get(user, repo, &"master".to_string(), &"rust-toolchain".to_string()).unwrap().trim().to_string();
        debug!("version from rust-toolchain: '{}'", version);
    }
    return Ok(version.to_string());
}

pub fn version_from_cargo(file: String) -> String {
    let re = Regex::new("^\\s*target\\s*[\"'](.*?)[\"']").unwrap();
    match re.captures(&file) {
        Some(caps) => caps.get(1).map_or("", |m| m.as_str()),
        None => ""
    }.to_string()
}

pub fn colour(version: &String) -> String {
    // Check version and set colour    
    match version.as_ref() {
        "1.25.0"         => "brightgreen",
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
        assert_eq!("red", rust::colour(&String::from("1.18.0")));
    }

    #[test]
    fn current_versions_are_green() {
        assert_eq!("brightgreen", rust::colour(&String::from("1.25.0")));
    }
}
