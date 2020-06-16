use sled;

pub fn insert(hash: String) {
    let tree = sled::open("./deals_db").expect("open");
    tree.insert(hash, "");
    tree.flush();
}
