pub fn raindrops(n: usize) -> String {
    let mut rd: String = "".to_string();
    let three: &str = "Pling";
    let five: &str = "Plang";
    let seven: &str = "Plong";

    if n % 3 == 0 {
        rd.push_str(three);
    }

    if n % 5 == 0 {
        rd.push_str(five);
    }

    if n % 7 == 0 {
        rd.push_str(seven);
    }

    if rd == "" {
        n.to_string()
    } else {
        rd
    }
}
