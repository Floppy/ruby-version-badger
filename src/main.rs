extern crate iron;
extern crate router;

mod config;

use iron::prelude::*;
use iron::status;
use router::Router;

fn main() {

    // build routes
    let mut router = Router::new();
    router.get("/", root, "index");
    router.get("/github/:user/:repo", github, "github");

    // serve
    let interface = format!("0.0.0.0:{}", config::port());
    println!("Binding on {:?}", interface);
    Iron::new(router).http(&interface[..]).unwrap();
    println!("Bound on {:?}", interface);

    // handlers
    fn root(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "root handler")))
    }

    fn github(req: &mut Request) -> IronResult<Response> {
        let ref user = req.extensions.get::<Router>().unwrap().find("user").unwrap_or("");
        let ref repo = req.extensions.get::<Router>().unwrap().find("repo").unwrap_or("");
        Ok(Response::with((status::Ok, "github handler")))
    }

}