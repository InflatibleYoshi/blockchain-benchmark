struct Blockchain{

}

enum ChainType{
    Cosmos,
    Hyperledger,
    Ethereum
}

struct Connection{
    stream: RwLock<TcpStream>
}

trait Api<'a> {
    //init account, get its address, add it to Node.
    fn init_account() -> &'static str;
    //send transaction through the blockchain api.
    fn send_transaction(transaction: Transaction) -> Result<(), &'static str>;


}

//TODO: figure out what information you need to interface with the API.

