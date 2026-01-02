fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    if let Err(e) = pkg_config::probe_library("libfoma") {
        eprintln!("pkg-config failed to find libfoma: {}", e);
        // Fallback to manual linking if pkg-config fails
        println!("cargo:rustc-link-lib=foma");
    }
}
