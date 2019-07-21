pub fn check(candidate: &str) -> bool {
    let mut letters = Vec::new();
    for i in candidate.chars() {
        // credit for the following one line found at: https://stackoverflow.com/questions/35432199/convert-a-char-to-upper-case
        let test: Vec<_> = i.to_lowercase().collect();
        if letters.contains(&test) && i != '-' && i != ' ' {
            return false;
        } else {
            letters.push(test);
        }
    }
    true
}
//will always return false since each letter compares itself.
