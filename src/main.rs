extern crate iron;
extern crate router;
extern crate staticfile;

extern crate futures;
extern crate hyper;
extern crate tokio_core;

mod config;
mod server;
mod handlers;

fn main() {
    server::serve();
}