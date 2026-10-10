use std::collections::{HashMap};
use std::collections::hash_map::Entry;

use crate::storage::{Storage};


pub struct TestStorage {
    data: Option<HashMap<String, String>>
}


impl TestStorage {
    pub fn new() -> Self {
        Self { data: None }
    }
}

impl Storage for TestStorage {
    fn get_token_by_domain(&self, domain: String) -> Option<String> {
        self.data.as_ref().and_then(|data| data.get(&domain).cloned())
    }

    fn create_domain(&mut self, domain: String) -> String {
        let token = String::from("token");
        let data = self.data.get_or_insert(HashMap::new());
        data.insert(domain, token.clone());
        token
    }

    fn remove_domain(&mut self, domain: String) {
        self.data.as_mut().and_then(|data| data.remove(&domain));
    }

    fn change_token(&mut self, domain: String) -> Option<String> {
        let mut entry = self.data.as_mut().map(|data| data.entry(domain));
        let new_token = String::from("token_new");
        if let Some(entry) = entry.as_mut()
            && let Entry::Occupied(entry) = entry {
                entry.insert(new_token.clone());
            }
        Some(new_token)
    }
}