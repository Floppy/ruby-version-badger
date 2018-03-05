#[macro_use] extern crate log;
extern crate env_logger;    
    
extern crate iron;
extern crate router;
extern crate staticfile;
extern crate regex;
extern crate reqwest;

mod config;
mod server;
mod handlers;
mod ruby;
mod rust;

fn main() {
    env_logger::init();
    info!("starting server");
    server::serve();
}
