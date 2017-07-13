use iron::prelude::*;
use router::Router;
use config;
use handlers;

pub fn serve() {
    
    // build routes
    let mut router = Router::new();
    router.get("/", handlers::root, "index");
    router.get("/github/:user/:repo", handlers::github, "github");

    // serve
    let interface = format!("0.0.0.0:{}", config::port());
    println!("Binding on {:?}", interface);
    Iron::new(router).http(&interface[..]).unwrap();
    println!("Bound on {:?}", interface);

}