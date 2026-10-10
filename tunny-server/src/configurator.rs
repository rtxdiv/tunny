use crate::storage::{Storage, test::TestStorage};


pub struct Config {
    pub port: u16,
    pub named_domains_storage: Option<Box<dyn Storage>>,
    pub api_config: Option<ApiConfig>,
    pub quic: bool
}

pub struct ApiConfig {
    pub login: String,
    pub password: String
}


impl Default for Config {
    fn default() -> Self {
        Self { port: 41013, named_domains_storage: None, api_config: None, quic: false }
    }
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn named_domains_testdb(&mut self) {
        self.named_domains_storage = Some(Box::new(TestStorage::new()));
    }
    pub fn allow_api(&mut self, login: String, password: String) {
        self.api_config = Some(ApiConfig { login, password });
    }
    pub fn allow_quic(&mut self) {
        self.quic = true;
    }
}