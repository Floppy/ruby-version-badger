use iron::prelude::*;
use router::Router;
use config;
use handlers;

pub fn serve() {    
    let interface = format!("0.0.0.0:{}", config::port());
    println!("Binding on {:?}", interface);
    Iron::new(router()).http(&interface[..]).unwrap();
    println!("Bound on {:?}", interface);
}

fn router () -> Router {
    let mut r = Router::new();
    r.get("/", handlers::root, "index");
    r.get("/github/:user/:repo", handlers::github, "github");    
    return r;
}