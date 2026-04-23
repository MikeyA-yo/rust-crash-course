pub fn num_to_string(num: u32) -> String {
    let x = match num {
        0 => "zero".to_string(),
        1 => "one".to_string(),
        2 => "two".to_string(),
        3 => "three".to_string(),
        _ => "other".to_string(),
    };
    x
}

pub fn unwrap_or_default(x: Option<u32>, v: u32) -> u32 {
    let x = match x {
        Some(x) => x,
        None => v,
    };
    x
}
