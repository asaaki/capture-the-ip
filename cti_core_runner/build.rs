fn main() {
    println!("cargo:rustc-link-lib=dylib=cti_core");
    println!("cargo:rustc-link-search=all=/app/bin");
}
