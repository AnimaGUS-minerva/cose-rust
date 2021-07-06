use std::env;
use std::fs;
use std::io::{Error, ErrorKind};

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

fn expose_under_target(src: &str, dir: Option<&str>, name: &str) -> std::io::Result<()> {
    let out_dir = env::var("OUT_DIR").unwrap(); // Not using `env!()` here; https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts
    let suffix = out_dir
        .rsplit("/target/")
        .collect::<Vec<_>>()[0];
    let target_dir = out_dir.replace(suffix, "");
    if !target_dir.ends_with("/target/") {
        return Err(Error::new(ErrorKind::Other, "Failed to resolve the 'target' dir."));
    }
    println!("@@ target_dir: {}", target_dir);

    let dest_dir = if let Some(dir) = dir {
        let dir_path = format!("{}{}/", target_dir, dir);
        fs::create_dir(&dir_path).unwrap_or_else(|why| {
            println!("! {:?}", why.kind());
        });
        dir_path
    } else {
        target_dir
    };
    println!("@@ dest_dir: {}", dest_dir);

    assert!(dest_dir.ends_with("/"));
    let src_out = format!("{}{}", dest_dir, name);

    let src = format!("{}/{}", env!("CARGO_MANIFEST_DIR"), src);
    println!("@@ src: {}\n  ----> src_out: {}", src, src_out);
    fs::copy(src, src_out)?;

    Ok(())
}
