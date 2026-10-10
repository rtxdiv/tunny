pub mod tcp;

pub trait StreamTable {
    async fn add();
    async fn remove();
    async fn send();
}