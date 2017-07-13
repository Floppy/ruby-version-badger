use futures::{Future, Stream};
use hyper::Client;
use hyper_rustls::HttpsConnector;
use tokio_core::reactor::Core;
use std::str;


pub fn get(url: String) -> String {
    // Set up the client
    let mut core = Core::new().unwrap();
    let handle = core.handle();    
    let client = Client::configure()
        .connector(HttpsConnector::new(4, &handle))
        .build(&handle);
    // Parse the URL
    let uri = url.parse().unwrap();
    // Set up the request and response handling as a future
    let request = client.get(uri).and_then(|resp| {
        // Get all the chunks until we have the entire response body
        resp.body().concat2()
    }).map(|chunk| str::from_utf8(&chunk).unwrap().to_string());
    // Actually run the damn thing
    core.run(request).unwrap()
}