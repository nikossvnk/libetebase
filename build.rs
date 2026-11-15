// SPDX-FileCopyrightText: © 2020 EteSync Authors
// SPDX-License-Identifier: LGPL-2.1-only

extern crate cbindgen;

use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    // Build scripts run on the host, so check the target through Cargo's variables, not cfg!()
    let target_family = env::var("CARGO_CFG_TARGET_FAMILY").unwrap_or_default();
    let target_vendor = env::var("CARGO_CFG_TARGET_VENDOR").unwrap_or_default();
    if target_family == "unix" && target_vendor != "apple" {
        // Set the soname for ELF platforms (not Windows or Apple's)
        const VERSION_MAJOR: &str = env!("CARGO_PKG_VERSION_MAJOR");
        println!(
            "cargo:rustc-cdylib-link-arg=-Wl,-soname,libetebase.so.{}",
            VERSION_MAJOR
        );
    }
    match cbindgen::generate(crate_dir) {
        Ok(gen) => gen,
        Err(e) => match e {
            // Ignore syntax errors because those will be handled later on by cargo build.
            cbindgen::Error::ParseSyntaxError {
                crate_name: _,
                src_path: _,
                error: _,
            } => return,
            _ => panic!("{:?}", e),
        },
    }
    .write_to_file("target/etebase.h");
}
