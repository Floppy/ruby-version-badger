use iron::prelude::*;
use iron::status;
use router::Router;

pub fn github(req: &mut Request) -> IronResult<Response> {
    
    // Get user and repo
    let ref user = req.extensions.get::<Router>().unwrap().find("user").unwrap_or("");
    let ref repo = req.extensions.get::<Router>().unwrap().find("repo").unwrap_or("");
        
    
    
    // Send response
    let str = format!("Generating badge for github.com/{}/{}", user, repo);
    Ok(Response::with((status::Ok, str)))
}
