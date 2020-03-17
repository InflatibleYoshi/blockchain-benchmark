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

//TODO: gRPC client/server that sends Payload messages to the nodes and receives Payload messages from each of the nodes.