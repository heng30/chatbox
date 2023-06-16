use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref CHAT_CACHE: Mutex<RefCell<HashMap<String, (String, String)>>> =
        Mutex::new(RefCell::new(HashMap::new()));
}

pub fn update_cache(suuid: String, uuid: String, text: String) {
    let item = CHAT_CACHE.lock().unwrap();
    let mut item = item.borrow_mut();
    if item.contains_key(&suuid) {
        let (cuuid, ctext) = item.get(&suuid).unwrap().clone();

        if cuuid == uuid {
            item.insert(suuid, (uuid, format!("{}{}", ctext, text)));
        } else {
            item.insert(suuid, (uuid, text));
        }
    } else {
        item.insert(suuid, (uuid, text));
    }
}

pub fn get_cache(suuid: &str) -> Option<(String, String)> {
    let item = CHAT_CACHE.lock().unwrap();
    let mut item = item.borrow_mut();
    item.remove(suuid)
}
