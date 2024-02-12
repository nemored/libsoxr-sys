extern crate pkg_config;

fn main() {
    // do not probe for libsoxr when compiling at docs.rs
    if std::env::var("DOCS_RS").is_ok() {
        return;
    }
    println!("cargo:rerun-if-env-changed=SOXR_LINK_SEARCH");
    if let Ok(s) = std::env::var("SOXR_LINK_SEARCH") {
            println!("cargo:rustc-link-search={s}");
            println!("cargo:rustc-link-lib=soxr");
            return;
    }
    if let Err(e) = pkg_config::probe_library("soxr") {
        match e {
            pkg_config::Error::Failure { .. } => panic! (
                "Pkg-config failed - usually this is because libsoxr development headers are not installed.\n\n\
                    For Mac users using brew: brew install libsoxr\n\n\
                    For Debian/Ubuntu users:\n# apt-get install libsoxr0-dev\n\n\
                    pkg_config details:\n{}",
                e
            ),
            pkg_config::Error::EnvNoPkgConfig(_) => (),
            _ => panic!("{}", e)
        }
    }
}
