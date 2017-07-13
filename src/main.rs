extern crate iron;

mod config;

use iron::prelude::*;
use iron::status;

fn main() {

    fn hello_world(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Hello World!")))
    }

    let url = format!("0.0.0.0:{}", config::port());

    println!("Binding on {:?}", url);
    Iron::new(hello_world).http(&url[..]).unwrap();
    println!("Bound on {:?}", url);
}