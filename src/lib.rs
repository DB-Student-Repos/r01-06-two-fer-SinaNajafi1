pub fn twofer(name: &str) -> String {
    let sn = "One for you, one for me.";
    let snt = "";
    if name == snt {
        return sn.to_string()
    } else {
        format!("One for {}, one for me.", name).to_string()
    }
}

