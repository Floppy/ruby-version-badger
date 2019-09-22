use iron::prelude::*;
use mount::Mount;
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

fn router () -> Mount {
    let mut r = Router::new();
    r.get("/:user/:repo", handlers::github, "github");

    let mut mount = Mount::new();
    mount
        .mount("/", Static::new(Path::new("static/index.html")))
        .mount("/github", r);
    return mount;
}
