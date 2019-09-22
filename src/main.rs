#[macro_use] extern crate log;
extern crate env_logger;

extern crate iron;
extern crate mount;
extern crate router;
extern crate staticfile;
extern crate regex;
extern crate reqwest;
extern crate yaml_rust;

mod config;
mod server;
mod handlers;
mod github;
mod ruby;
mod rust;
mod node;

fn main() {
    env_logger::init();
    info!("starting server");
    server::serve();
}
