use iron::prelude::*;
use iron::{Url, status};
use iron::modifiers::Redirect;
use router::Router;
use https;

use regex::Regex;

pub fn github(req: &mut Request) -> IronResult<Response> {
    
    // Get user and repo
    let ref user = req.extensions.get::<Router>().unwrap().find("user").unwrap_or("");
    let ref repo = req.extensions.get::<Router>().unwrap().find("repo").unwrap_or("");
        
    // Get a file
    let url = String::from(format!("https://raw.githubusercontent.com/{}/{}/master/Gemfile", user, repo));
    let gemfile = https::get(url);
    
    // Get ruby version
    let version = parse_gemfile(gemfile);
    
    // Create URL
    let badge = format!("https://img.shields.io/badge/ruby-{}-lightgray.svg", version);
    let badge_url = Url::parse(&badge).unwrap();
    
    // Send response
    Ok(Response::with((status::Found, Redirect(badge_url))))
}

fn parse_gemfile(gemfile: String) -> String {
    let re = Regex::new("ruby [\"\'](.*?)[\"\']").unwrap();
    let caps = re.captures(&gemfile).unwrap();
    let str = caps.get(1).map_or("", |m| m.as_str());
    String::from(str)
}