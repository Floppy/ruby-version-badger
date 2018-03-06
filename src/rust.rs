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

pub fn version(user: &String, repo: &String) -> Result<String, reqwest::Error> {
    let mut version = "unknown".to_string();
    // Get ruby version from Gemfile
    let url = format!("https://raw.githubusercontent.com/{}/{}/master/Cargo.toml", user, repo);
    let file = reqwest::get(url.as_str())?.text()?;
    version = version_from_cargo(file);
    debug!("version from Cargo.toml: '{}'", version);    
    // fall back to rust-toolchain
    if version == "" {
        // Get a file
        let url = format!("https://raw.githubusercontent.com/{}/{}/master/rust-toolchain", user, repo);
        version = reqwest::get(url.as_str())?.text()?.trim().to_string();
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
        "1.24.1"         => "brightgreen",
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
        assert_eq!("brightgreen", rust::colour(String::from("1.24.1")));
    }
}
