pub fn bedazzle(s: &mut String) {
    *s = "sparkly".to_string();
}

pub fn change(s: &mut String) {
    if !s.ends_with("s") {
        s.push_str("s");
    }
}

pub fn eat(s: String) -> bool {
    s.starts_with("b") && s.contains("a")
}

pub fn inspect(s: &String) {
    if s.ends_with("s") {
        println!("Plural");
    } else {
        println!("Singular");
    }
}
