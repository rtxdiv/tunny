pub mod configurator;
pub mod storage;

pub fn run(config: &mut configurator::Config) {
    if let Some(ref mut storage) = config.named_domains_storage {
        let domain = &String::from("domain");
        storage.create_domain(domain);

        let token = storage.get_token_by_domain(domain);
        println!("{token:?}");

        let new_token = storage.change_token(domain);
        println!("{new_token:?}");
    }
}