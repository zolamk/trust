pub fn version() {
    println!("trust-version {}", env!("CARGO_PKG_VERSION").to_string());
}
