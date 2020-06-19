use regex::Regex;
use crate::github;
use reqwest;

pub fn detected(user: &String, repo: &String) -> Result<bool, reqwest::Error> {
    return github::exists(
        user,
        repo,
        &"master".to_string(),
        &"package.json".to_string()
    );
}
pub fn name() -> String {
    return "rust".to_string();
}

pub fn version(user: &String, repo: &String) -> Result<String, reqwest::Error> {
    let mut version = "unknown".to_string();
    // Get ruby version from Gemfile
    let file = github::get(user, repo, &"master".to_string(), &"package.json".to_string());
    version = version_from_package_json(file.unwrap());
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
        "12.10.0"        => "brightgreen",
        "10.16.3"        => "brightgreen",
        "8.16.1"         => "orange",
        ""               => "lightgray",
        "404: Not Found" => "lightgray",
        _                => "red",
    }.to_string()
}
