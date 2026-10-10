use std::collections::HashMap;
use crate::streams::StreamTable;


struct TcpTable(HashMap<String, i32>);

impl StreamTable for TcpTable {
    async fn add() { todo!() }
    async fn remove() { todo!() }
    async fn send() { todo!() }
}

pub fn start_tcp_listener() {

}