pub fn reverse(input: &str) -> String {
    let mut reversed: String = "".to_string();
    for c in input.chars().rev() {
        reversed.push(c);
    }
    reversed
}
