use tonic::codec::Codec;

struct Payload<'a> {
    transactions: HashMap<SystemTime, Transaction<'a>>
}

struct Transaction<'a>{
    id: u32,
    send_address: &'a str,
    recv_address: &'a str,
    time: SystemTime
}

struct SmartContract{

}

//TODO: generate payload of transactions.
impl<'a> Codec for Payload<'a>{

}

#[derive(Debug)]
struct PayloadService;

#[tonic::async_trait]
impl PayloadGrpc for PayloadService {

    type PayloadStream = Pin<Box<dyn Stream<Item = Result<Transaction, Status>> + Send + Sync + 'static>>;

    async fn benchmark_payload(&self, _request: Request<tonic::Streaming<Transaction>>) -> Result<Response<Self::PayloadStream>, Status> {
        unimplemented!()
    }

    async fn get_status(&self, request: Request<JobStatus>) -> Result<Response<JobStatus>, Status> {
        unimplemented!()
    }

}