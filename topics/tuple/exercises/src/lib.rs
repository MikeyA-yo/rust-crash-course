pub fn first(t: (bool, u32, char)) -> bool {
    let (a, _, _) = t;
    a
}

pub fn last(t: (bool, u32, char)) -> char {
    let (_, _, c) = t;
    c
}

pub fn swap(t: (u32, u32)) -> (u32, u32) {
    let (a, b) = t;
    (b, a)
}
