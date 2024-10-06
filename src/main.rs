pub const FULL_VERSION: &str = concat!(env!("CARGO_PKG_VERSION"), "-", env!("VERGEN_GIT_DESCRIBE"));

fn main() {
    println!("Hello, {}", FULL_VERSION);
}
