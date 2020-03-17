struct Payload {
    transactions: HashMap<SystemTime, Transaction>
}

struct Transaction{
    send_address: u64,
    recv_address: u64,
    time: SystemTime
}

struct SmartContract{

}

//TODO: generate payload of transactions.