extern crate iron;
extern crate router;
extern crate staticfile;

mod config;
mod server;
mod handlers;

fn main() {
    server::serve();
}