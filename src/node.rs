use regex::Regex;
use github;
use reqwest;

pub fn detected(user: &String, repo: &String) -> Result<bool, reqwest::Error> {
    return github::exists(
        user, 
        repo, 
        &"master".to_string(), 
        &"package.json".to_string()
    );
}

pub fn version(user: &String, repo: &String) -> Result<String, reqwest::Error> {
    let mut version = "unknown".to_string();
    // Get ruby version from Gemfile
    let url = format!("https://raw.githubusercontent.com/{}/{}/master/package.json", user, repo);
    let file = reqwest::get(url.as_str())?.text()?;
    version = version_from_package_json(file);
    debug!("version from package.json: '{}'", version);    
    return Ok(version.to_string());
}

pub fn version_from_package_json(file: String) -> String {
    let re = Regex::new("[\"']node[\"']:\\s*[\"'](.*?)[\"']").unwrap();
    match re.captures(&file) {
        Some(caps) => caps.get(1).map_or("", |m| m.as_str()),
        None => ""
    }.to_string()
}

pub fn colour(version: &String) -> String {
    // Check version and set colour    
    match version.as_ref() {
        "9.7.1"          => "brightgreen",
        "8.9.4"          => "brightgreen",
        "6.13.0"         => "yellow",
        ""               => "lightgray",
        "404: Not Found" => "lightgray",
        _                => "red",
    }.to_string()
}
