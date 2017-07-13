use iron::prelude::*;
use iron::status;
use router::Router;
use https;

pub fn github(req: &mut Request) -> IronResult<Response> {
    
    // Get user and repo
    let ref user = req.extensions.get::<Router>().unwrap().find("user").unwrap_or("");
    let ref repo = req.extensions.get::<Router>().unwrap().find("repo").unwrap_or("");
        
    // Get a file
    let url = String::from(format!("https://raw.githubusercontent.com/{}/{}/master/Gemfile", user, repo));
    let gemfile = https::get(url);
    
    // Send response
    Ok(Response::with((status::Ok, gemfile)))
}