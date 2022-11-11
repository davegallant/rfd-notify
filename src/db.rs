pub fn get_config(dbpath: &str) -> sled::Config {
    sled::Config::default()
        .path(dbpath)
        .cache_capacity(100_000_000)
        .flush_every_ms(Some(1000))
}

pub fn hash_exists(hash: &str, config: sled::Config) -> bool {
    let tree = config.open().unwrap();
    let result = tree.get(hash);
    if result.is_err() {
        error!("{:?}", &result);
    }
    if result == Ok(None) {
        return false;
    }
    true
}

pub fn insert(hash: &str, config: sled::Config) {
    let tree = config.open().unwrap();
    let result = tree.insert(hash, "");
    if result.is_err() {
        error!("{:?}", &result);
    }
    let result = tree.flush();
    if result.is_err() {
        error!("{:?}", &result);
    }
}
