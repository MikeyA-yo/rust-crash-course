use std::collections::HashMap;

pub fn filter_non_zero(v: Vec<u32>) -> Vec<u32> {
    let res: Vec<u32> = v.into_iter().filter(|x| *x != 0).collect();
    res
}

pub fn to_string(v: Vec<&str>) -> Vec<String> {
    let res = v.into_iter().map(|x| String::from(x)).collect();
    res
}

pub fn to_hash_map(v: Vec<(String, u32)>) -> HashMap<String, u32> {
    let hm: HashMap<String, u32> = v.into_iter().map(|x| (x.0, x.1)).collect();
    hm
}
