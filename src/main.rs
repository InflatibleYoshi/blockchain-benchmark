/// ## Usage
///
/// Run the example:
///
///     cargo run --example hello_world
///
/// Then send a request:
///
///     curl -v http://localhost:8080/

#[macro_use]
extern crate tower_web;

use tower_web::codegen::futures::*;
use tower_web::ServiceBuilder;
use tokio::prelude::*;

#[derive(Clone, Debug)]
pub struct HelloWorld {
    /// The message that will be included in the response to `/motd`.
    motd: String,
}

impl_web! {
    impl HelloWorld {

        #[get("/")]
        fn hello_world(&self) -> Result<&'static str, ()> {
            Ok("This is a basic response served by tower-web")
        }

        #[post("/")]
        fn hello_world(&self) -> Result<&'static str, ()> {
            Ok("This is a basic response served by tower-web")
        }
    }
}

pub fn main() {
    // Next, we must run our web service.
    //
    // The HTTP service will listen on this address and port.
    let addr = "127.0.0.1:8080".parse().expect("Invalid address");
    println!("Listening on http://{}", addr);

    // A service builder is used to configure our service.
    ServiceBuilder::new()
        // We add the resources that are part of the service. In this example,
        // there is only a single resource.
        .resource(HelloWorld {
            motd: "tower-web is amazing!!!".to_string(),
        })
        // We run the service
        .run(&addr)
        .unwrap();


}
