pub mod test;

pub trait Storage {
    fn get_token_by_domain(&self, domain: &str) -> Option<String>;
    fn create_domain(&mut self, domain: &str) -> String;
    fn remove_domain(&mut self, domain: &str);
    fn change_token(&mut self, domain: &str) -> Option<String>;
}
