pub fn first_subword(mut s: String) -> String {
    for (index, ch) in s.char_indices() {
        if ch == '_' || ch.is_uppercase() && index != 0{
            s.truncate(index);
            return s;
        }
    }

    return s;

}
