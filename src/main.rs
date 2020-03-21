


pub fn main() {
    tonic_build::compile_protos("proto/payload.proto")
        .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));

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
