use reqwest;

pub fn exists(user: &String, repo: &String, branch: &String, filename: &String) -> Result<bool, reqwest::Error> {
    let url = format!("https://raw.githubusercontent.com/{}/{}/{}/{}", user, repo, branch, filename);
    let resp = reqwest::get(url.as_str())?;
    return Ok(resp.status().is_success());
}
