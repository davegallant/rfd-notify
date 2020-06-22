fn get_config() -> sled::Config {
    sled::Config::default()
        .path("./deals_db".to_owned())
        .cache_capacity(100_000_000)
        .flush_every_ms(Some(1000))
}

pub fn hash_exists(hash: &str) -> bool {
    let db = get_config();
    let tree = db.open().unwrap();
    let result = tree.get(hash);
    if result.is_err() {
        error!("{:?}", &result);
    }
    if result == Ok(None) {
        return false;
    }
    true
}

pub fn insert(hash: &str) {
    let db = get_config();
    let tree = db.open().unwrap();
    let result = tree.insert(&hash, "");
    if result.is_err() {
        error!("{:?}", &result);
    }
    let result = tree.flush();
    if result.is_err() {
        error!("{:?}", &result);
    }
}
