use regex::Regex;
use reqwest;

pub fn detected(user: &String, repo: &String) -> Result<bool, reqwest::Error> {
    let url = format!("https://raw.githubusercontent.com/{}/{}/master/Gemfile", user, repo);
    let resp = reqwest::get(url.as_str())?;
    return Ok(resp.status().is_success());
}

pub fn version(user: &String, repo: &String) -> Result<String, reqwest::Error> {
    let mut version = "unknown".to_string();
    // Get ruby version from Gemfile
    let url = format!("https://raw.githubusercontent.com/{}/{}/master/Gemfile", user, repo);
    let gemfile = reqwest::get(url.as_str())?.text()?;
    version = version_from_gemfile(gemfile);
    debug!("version from Gemfile: '{}'", version);    
    // fall back to .ruby-version
    if version == "" {
        // Get a file
        let url = format!("https://raw.githubusercontent.com/{}/{}/master/.ruby-version", user, repo);
        version = reqwest::get(url.as_str())?.text()?.trim().to_string();
        debug!("version from .ruby-version: '{}'", version);
    }
    return Ok(version.to_string());
}

pub fn colour(version: &String) -> String {
    // Check version and set colour    
    match version.as_ref() {
        "2.5.0"          => "brightgreen",
        "2.4.3"          => "yellow",
        "2.3.6"          => "yellow",
        "2.2.9"          => "orange",
        ""               => "lightgray",
        "404: Not Found" => "lightgray",
        _                => "red",
    }.to_string()
}

pub fn version_from_gemfile(gemfile: String) -> String {
    let re = Regex::new("^\\s*ruby\\s*[\"'](.*?)[\"']").unwrap();
    match re.captures(&gemfile) {
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
        assert_eq!("brightgreen", ruby::colour(&String::from("2.5.0")));
    }

    #[test]
    fn supported_versions_are_yellow() {
        assert_eq!("yellow", ruby::colour(&String::from("2.3.6")));
    }

    #[test]
    fn ending_versions_are_orange() {
        assert_eq!("orange", ruby::colour(&String::from("2.2.9")));
    }
}
