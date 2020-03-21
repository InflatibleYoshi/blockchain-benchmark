use tonic::codec::Codec;

struct Payload<'a> {
    transactions: HashMap<&'a str, Vec<Transaction<'a>>>
}

#[derive(Hash, Clone, Debug, Deserialize)]
struct Transaction<'a>{
    id: u32,
    send_address: &'a str,
    recv_address: &'a str,
    time: SystemTime
}

#[derive(Debug, Deserialize)]
enum JobStatus{
    QUERY,
    AVAILABLE,
    OCCUPIED,
    FINISHED
}

struct SmartContract{

}

//TODO: generate payload of transactions.
impl<'a> Codec for Payload<'a>{

}

#[derive(Debug)]
struct PayloadWrapper;

//TODO: gRPC client/server that sends Payload messages to the nodes and receives Payload messages from each of the nodes.
#[tonic::async_trait]
impl PayloadService for PayloadWrapper {

    type PayloadStream = Pin<Box<dyn Stream<Item = Result<Transaction, Status>> + Send + Sync + 'static>>;

    async fn benchmark_payload(&self, _request: Request<tonic::Streaming<Transaction>>) -> Result<Response<Self::PayloadStream>, Status> {
        let mut stream = request.into_inner();

        let mut transactions = HashMap::new();

        let output = async_stream::try_stream! {
            while let Some(transaction) = stream.next().await {
                let transaction = transaction?.clone();

                let all_transactions = transactions.entry(transactions).or_insert(vec![]);
                all_transactions.push(transaction);

                for note in location_notes {
                    yield note.clone();
                }
            }
        };

        Ok(Response::new(Box::pin(output) as Self::PayloadStream))
    }

    type JobStatusStream = mpsc::Receiver<Result<JobStatus, Status>>;

    async fn get_status(&self, request: Request<JobStatus>) -> Result<Response<JobStatus>, Status> {

        let (mut tx, rx) = mpsc::channel(4);
        let features = self.features.clone();

        tokio::spawn(async move {
            for feature in &features[..] {
                if in_range(feature.location.as_ref().unwrap(), request.get_ref()) {
                    println!("  => send {:?}", feature);
                    tx.send(Ok(feature.clone())).await.unwrap();
                }
            }

        });

        Ok(Response::new(rx))
    }

}