use std::collections::*;

#[no_mangle]
pub extern "C" fn btreemap() -> BTreeMap<String, String> {
   BTreeMap::new()
}

#[no_mangle]
pub extern "C" fn vec() -> Vec<String> {
   Vec::new()
}

#[no_mangle]
pub extern "C" fn hashmap() -> HashMap<String, String> {
   HashMap::new()
}
