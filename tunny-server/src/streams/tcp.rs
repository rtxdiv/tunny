use std::{collections::HashMap, sync::Arc};
use crate::streams::StreamTable;
use tokio::sync::{mpsc, Mutex};

type PeerMap = HashMap<String, mpsc::UnboundedSender<Vec<u8>>>;

struct TcpTable {
    registry: Arc<Mutex<PeerMap>>
}

impl StreamTable for TcpTable {
    async fn add() { todo!() }
    async fn remove() { todo!() }
    async fn send() { todo!() }
}

pub fn start_tcp_listener() {

}