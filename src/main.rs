#[macro_use] extern crate log;
extern crate env_logger;    
    
extern crate iron;
extern crate router;
extern crate staticfile;
extern crate regex;

extern crate futures;
extern crate hyper;
extern crate hyper_rustls;
extern crate tokio_core;

mod config;
mod https;
mod server;
mod handlers;
mod ruby;
mod rust;

fn main() {
    env_logger::init();
    info!("starting server");
    server::serve();
}