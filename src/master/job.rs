#[derive(Clone, Debug)]
struct Job{
    job_type: PayloadType,
    time: SystemTime,

}

enum PayloadType{

}

trait Ledger {
    fn generate_payload() -> Payload();
    fn generate_contract() -> SmartContract();
    fn generate_results() -> Benchmark();
}

#[macro_use]
extern crate tower_web;

use tower_web::codegen::futures::*;
use tower_web::ServiceBuilder;
use tokio::prelude::*;

impl_web! {
    impl Job {

        #[get("/")]
        fn hello_world(&self) -> Result<&'static str, ()> {
            Ok("This is a basic response served by tower-web")
        }

        #[post("/job")]
        fn hello_world(&self) -> Result<&'static str, ()> {
            Ok("This is a basic response served by tower-web")
        }
    }
}

