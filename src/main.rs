extern crate iron;

use iron::prelude::*;
use iron::status;
use std::env;

fn main() {

    fn hello_world(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Hello World!")))
    }

    let mut port = String::from("3000");
    match env::var("PORT") {
        Ok(val) => port = val,
        Err(_e) => println!("using default port {}",  port),
    }

    let url = format!("0.0.0.0:{}", port);

    println!("Binding on {:?}", url);
    Iron::new(hello_world).http(&url[..]).unwrap();
    println!("Bound on {:?}", url);
}