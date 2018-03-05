use iron::prelude::*;
use iron::{Url, status};
use iron::modifiers::Redirect;
use router::Router;
use ruby;

pub fn github(req: &mut Request) -> IronResult<Response> {
    
    // Get user and repo
    let user = req.extensions.get::<Router>().unwrap().find("user").unwrap_or("").to_string();
    let repo = req.extensions.get::<Router>().unwrap().find("repo").unwrap_or("").to_string();
        
    let mut version = "unknown".to_string();
    let mut language = "language".to_string();
    let mut colour = "lightgray".to_string();

    // Detect language
    if ruby::detected(&user, &repo).unwrap() {
        language = "ruby".to_string();
        version = ruby::version(&user, &repo).unwrap();
        colour = ruby::colour(&version);
    }

    // Create URL (without dashes in the version)
    let badge = format!("https://img.shields.io/badge/{}-{}-{}.svg", language, version.replace("-", ""), colour);
    let badge_url = Url::parse(&badge).unwrap();
    
    // Send response
    Ok(Response::with((status::Found, Redirect(badge_url))))
}
