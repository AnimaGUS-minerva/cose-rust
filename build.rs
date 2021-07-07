use std::env;

extern crate minerva_refract;
use minerva_refract::expose_under_target;

fn main() {
    // Use NSS_LIB_DIR lazily. If it's not set and we can't find NSS in the path,
    // the build will fail.
    #[cfg(test)]
    let lib_dir = env::var("NSS_LIB_DIR");
    if let Ok(lib_dir) = env::var("NSS_LIB_DIR") {
        println!("cargo:rustc-link-search={}", lib_dir);
    }

    println!("cargo:rerun-if-changed=src/test_setup.rs");
    println!("cargo:rerun-if-changed=src/test_cose.rs");
    expose_under_target("src/test_setup.rs", Some("expose_cose"), "test_setup.rs").unwrap();
    expose_under_target("src/test_cose.rs", Some("expose_cose"), "test_cose.rs").unwrap();
}
