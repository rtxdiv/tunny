use crate::storage::{Storage, test::TestStorage};


pub struct Config {
    pub tcp_port: Option<u16>,
    pub quic_port: Option<u64>,
    pub named_domains_storage: Option<Box<dyn Storage>>,
    pub api_config: Option<ApiConfig>,
}

pub struct ApiConfig {
    pub login: String,
    pub password: String
}


impl Default for Config {
    fn default() -> Self {
        Self { tcp_port: Some(41013), quic_port: Some(41014), named_domains_storage: None, api_config: None }
    }
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn new_tcp() -> Self {
        Self { tcp_port: Some(41013), quic_port: None, named_domains_storage: None, api_config: None }
    }
    pub fn new_quic() -> Self {
        Self { tcp_port: None, quic_port: Some(41014), named_domains_storage: None, api_config: None }
    }
    
    pub fn named_domains_testdb(&mut self) {
        self.named_domains_storage = Some(Box::new(TestStorage::new()));
    }
    pub fn allow_api(&mut self, login: String, password: String) {
        self.api_config = Some(ApiConfig { login, password });
    }
}