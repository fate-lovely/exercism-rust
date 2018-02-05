pub fn raindrops(n: usize) -> String {
    let mut out = String::new();

    if n % 3 == 0 {
        out += "Pling";
    }

    if n % 5 == 0 {
        out += "Plang";
    }

    if n % 7 == 0 {
        out += "Plong";
    }

    if out == "" {
        out = n.to_string();
    }

    out
}
