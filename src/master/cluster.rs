struct Cluster{
    nodes: Arc<Vec<Node>>
}


struct Node{
    id: i32,
    address: &'static str,
    ip: SocketAddr,

}

