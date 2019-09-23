use regex::Regex;
use github;
use reqwest;

pub fn detected(user: &String, repo: &String) -> Result<bool, reqwest::Error> {
    return github::exists(
        user,
        repo,
        &"master".to_string(),
        &"Gemfile".to_string()
    );
}

pub fn name() -> String {
    return "ruby".to_string();
}

pub fn version(user: &String, repo: &String) -> Result<String, reqwest::Error> {
    let mut version = "unknown".to_string();
    // Get ruby version from Gemfile
    let file = github::get(user, repo, &"master".to_string(), &"Gemfile".to_string());
    version = version_from_gemfile(file.unwrap());
    debug!("version from Gemfile: '{}'", version);
    // fall back to .ruby-version
    if version == "" {
        // Get a file
        version = github::get(user, repo, &"master".to_string(), &".ruby-version".to_string()).unwrap().trim().to_string();
        debug!("version from .ruby-version: '{}'", version);
    }
    return Ok(version.to_string());
}

pub fn colour(version: &String) -> String {
    // Check version and set colour
    match version.as_ref() {
        "2.6.4"          => "brightgreen",
        "2.5.6"          => "yellow",
        "2.4.7"          => "orange",
        ""               => "lightgray",
        "404: Not Found" => "lightgray",
        _                => "red",
    }.to_string()
}

pub fn version_from_gemfile(file: String) -> String {
    let re = Regex::new("^\\s*ruby\\s*[\"'](.*?)[\"']").unwrap();
    match re.captures(&file) {
        Some(caps) => caps.get(1).map_or("", |m| m.as_str()),
        None => ""
    }.to_string()
}

#[cfg(test)]
mod tests {
    use ruby;

    #[test]
    fn deprecated_versions_are_red() {
        assert_eq!("red", ruby::colour(&String::from("1.9.3")));
    }

    #[test]
    fn current_versions_are_green() {
        assert_eq!("brightgreen", ruby::colour(&String::from("2.6.4")));
    }

    #[test]
    fn supported_versions_are_yellow() {
        assert_eq!("yellow", ruby::colour(&String::from("2.5.6")));
    }

    #[test]
    fn ending_versions_are_orange() {
        assert_eq!("orange", ruby::colour(&String::from("2.4.7")));
    }
}
