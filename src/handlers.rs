use iron::prelude::*;
use iron::status;
use router::Router;

pub fn root(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "root handler")))
}

pub fn github(req: &mut Request) -> IronResult<Response> {
    let ref user = req.extensions.get::<Router>().unwrap().find("user").unwrap_or("");
    let ref repo = req.extensions.get::<Router>().unwrap().find("repo").unwrap_or("");
    Ok(Response::with((status::Ok, "github handler")))
}
