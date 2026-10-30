pub fn is_empty(v: &str) -> bool {
    return v.chars().count() == 0;
}

pub fn is_ascii(v: &str) -> bool {
    for b in v.bytes() {
        if (b <= 0 && b > 255) {
            return false;
        }
    } 
    return true;
}

pub fn contains(v: &str, pat: &str) -> bool {
    return v.contains();
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
    return (&v[..index], &v[index..]);
}

pub fn find(v: &str, pat: char) -> usize {
    for (index, ch) in v.char_indices() {
        if ch == pat {
            return index;
        }
    }
    v.len() 
}
