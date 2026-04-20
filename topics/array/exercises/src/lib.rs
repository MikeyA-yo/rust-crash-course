pub fn zeros() -> [u32; 100] {
    let b = [0; 100];
    b
}

pub fn first_3(s: &[u32]) -> &[u32] {
    &s[..3]
}

pub fn last_3(s: &[u32]) -> &[u32] {
    &s[s.len() - 3..]
}
