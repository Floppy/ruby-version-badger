extern crate iron;
extern crate router;

mod config;
mod server;
mod handlers;

fn main() {
    server::serve();
}