use std::collections::HashMap;

pub fn init(address: String, amount: u32) -> HashMap<String, u32> {
    let mut add = HashMap::new();
    add.insert(address, amount);
    add
}
