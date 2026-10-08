use crate::storage::{Storage, test::TestStorage};


pub struct Config {
    pub port: u16,
    pub named_domains: bool,
    pub named_domains_storage: Option<Box<dyn Storage>>,
    pub panel_config: Option<PanelConfig>
}

pub struct PanelConfig {
    pub login: String,
    pub password: String
}


impl Default for Config {
    fn default() -> Self {
        Self { port: 41013, named_domains: false, named_domains_storage: None, panel_config: None }
    }
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn named_domains_testdb(&mut self) {
        self.named_domains = true;
        self.named_domains_storage = Some(Box::new(TestStorage::new(String::from("cooltext"))));
    }
}