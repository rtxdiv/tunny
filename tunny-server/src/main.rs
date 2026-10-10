use tunny_server::configurator;

fn main() {
    let mut config = configurator::Config::new();
    config.named_domains_testdb();
    tunny_server::run(&mut config);
}
