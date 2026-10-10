pub mod test;

pub trait Storage {
    fn get_token_by_domain(&self, domain: &String) -> Option<String>;
    fn create_domain(&mut self, domain: &String) -> String;
    fn remove_domain(&mut self, domain: &String);
    fn change_token(&mut self, domain: &String) -> Option<String>;
}

