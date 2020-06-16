use sled;

pub fn hash_exists(hash: &String) -> bool {
    let tree = sled::open("./deals_db").expect("open");
    let result = tree.get(hash);
    if result.is_err() {
        error!("{:?}", &result);
    }
    if result == Ok(None) {
        return false;
    }
    return true;
}

pub fn insert(hash: &String) {
    let tree = sled::open("./deals_db").expect("open");
    let result = tree.insert(&hash, "");
    if result.is_err() {
        error!("{:?}", &result);
    }
    let result = tree.flush();
    if result.is_err() {
        error!("{:?}", &result);
    }
}
