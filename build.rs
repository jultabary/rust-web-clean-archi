fn main() {
    // see https://docs.rs/sqlx/latest/sqlx/macro.migrate.html#stable-rust-cargo-build-script
    println!("cargo:rerun-if-changed=migrations");
}
