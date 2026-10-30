pub fn initials(names: Vec<&str>) -> Vec<String> {
    names
        .into_iter()
        .map(|name| {
            let chars: Vec<String> = name
                .split_whitespace()
                .filter_map(|word| word.chars().next())
                .map(|c| c.to_uppercase().to_string())
                .collect();
            
            chars.join(" ")
        })
        .collect()
}