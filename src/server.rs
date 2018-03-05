use iron::prelude::*;
use router::Router;
use staticfile::Static;
use std::path::Path;
use config;
use handlers;

pub fn serve() {    
    let interface = format!("0.0.0.0:{}", config::port());
    info!("Binding on {:?}", interface);
    Iron::new(router()).http(&interface[..]).unwrap();
    info!("Bound on {:?}", interface);
}

fn router () -> Router {
    let mut r = Router::new();
    r.get("/", Static::new(Path::new("static/index.html")), "root");
    r.get("/github/:user/:repo", handlers::github, "github");    
    return r;
}
