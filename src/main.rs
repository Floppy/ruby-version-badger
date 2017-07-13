extern crate iron;

use iron::prelude::*;
use iron::status;

use std::env;

fn port() -> String {
    let mut p = String::from("3000");
    match env::var("PORT") {
        Ok(val) => p = val,
        Err(_e) => println!("using default port {}",  p),
    }
    return p;
}

fn main() {

    fn hello_world(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Hello World!")))
    }

    let url = format!("0.0.0.0:{}", ::port());

    println!("Binding on {:?}", url);
    Iron::new(hello_world).http(&url[..]).unwrap();
    println!("Bound on {:?}", url);
}