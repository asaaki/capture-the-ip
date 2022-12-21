fn main() {
    // The library we want to depend on
    println!("cargo:rustc-link-lib=dylib=cti_core");
    // The (extra) search path/location to look for the library
    println!("cargo:rustc-link-search=all=/app/bin");
    // Note: the final binaries do not depend on the specific fixed path here.
    // I believe the dynamic loader (ld-…) is searching for libraries in all
    // known locations (like /lib and /usr/lib), or extra locations advertised
    // via LD_LIBRARY_PATH env var.
    // To avoid the latter we place cti_core in /lib for ease of use.
}
