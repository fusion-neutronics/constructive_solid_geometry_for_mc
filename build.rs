// build.rs
fn main() {
    // If the python feature is enabled, tell cargo we're using the python feature
    if std::env::var("CARGO_FEATURE_PYTHON").is_ok() {
        println!("cargo:rustc-cfg=feature=\"python\"");
        // We could also dynamically add cdylib to crate-type here, but we'll use the approach
        // with conditional crate-type in Cargo.toml instead
    }
}
