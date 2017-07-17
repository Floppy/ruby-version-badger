use iron::prelude::*;
use iron::{Url, status};
use iron::modifiers::Redirect;
use router::Router;
use https;
use ruby;

pub fn github(req: &mut Request) -> IronResult<Response> {
    
    // Get user and repo
    let ref user = req.extensions.get::<Router>().unwrap().find("user").unwrap_or("");
    let ref repo = req.extensions.get::<Router>().unwrap().find("repo").unwrap_or("");
        
    // Get ruby version from Gemfile
    let url = String::from(format!("https://raw.githubusercontent.com/{}/{}/master/Gemfile", user, repo));
    let gemfile = https::get(url);
    let mut version = ruby::version_from_gemfile(gemfile);
    println!("version from Gemfile: '{}'", version);
    
    // fall back to .ruby-version
    if version == "" {
        // Get a file
        let url = String::from(format!("https://raw.githubusercontent.com/{}/{}/master/.ruby-version", user, repo));
        version = String::from(https::get(url).trim());
        println!("version from .ruby-version: '{}'", version);
    }

    let colour = ruby::colour(version.to_string());
    if colour == "lightgray" {
        version = String::from("unknown");
    }

    // Create URL (without dashes in the version)
    let badge = format!("https://img.shields.io/badge/ruby-{}-{}.svg", version.replace("-", ""), colour);
    let badge_url = Url::parse(&badge).unwrap();
    
    // Send response
    Ok(Response::with((status::Found, Redirect(badge_url))))
}