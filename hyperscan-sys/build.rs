#[cfg(feature = "gen")]
extern crate bindgen;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate pkg_config;

use std::env;
#[cfg(not(feature = "gen"))]
use std::fs;
use std::path::{Path, PathBuf};

struct Library {
    pub libs: Vec<String>,
    pub link_paths: Vec<PathBuf>,
    pub include_paths: Vec<PathBuf>,
}

fn find_hyperscan() -> Library {
    if let Ok(prefix) = env::var("HYPERSCAN_ROOT") {
        debug!("building with Hyperscan @ HYPERSCAN_ROOT={}", prefix);

        Library {
            libs: vec![From::from("hs")],
            link_paths: vec![From::from(format!("{}/lib", prefix))],
            include_paths: vec![From::from(format!("{}/include", prefix))],
        }
    } else if let Ok(pkg_config::Library {
        libs,
        link_paths,
        include_paths,
        ..
    }) = pkg_config::Config::new().statik(true).probe("libhs")
    {
        debug!(
            "building with Hyperscan @ libs={:?}, link_paths={:?}, include_paths={:?}",
            libs, link_paths, include_paths
        );

        Library {
            libs: libs,
            link_paths: link_paths,
            include_paths: include_paths,
        }
    } else {
        panic!("please install hyperscan from https://github.com/01org/hyperscan")
    }
}

#[cfg(feature = "gen")]
fn generate_binding(hyperscan_include_path: &str, out_file: &Path) {
    info!("generating raw Hyperscan wrapper @ {}", out_file.display());

    bindgen::builder()
        .header(format!("{}/hs.h", hyperscan_include_path))
        .clang_arg("-xc++")
        .clang_arg("-std=c++11")
        .whitelist_function("^hs_.*")
        .derive_default(true)
        .generate()
        .expect("Fail to generate bindings")
        .write_to_file(out_file)
        .expect("Fail to write raw wrapper");

    println!("cargo:rerun-if-changed={}/hs.h", hyperscan_include_path);
}

#[cfg(not(feature = "gen"))]
fn generate_binding(_: &str, out_file: &Path) {
    let target_os   = std::env::var("CARGO_CFG_TARGET_OS"  ).unwrap();
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();

    let bindings = match (target_os.as_str(), target_arch.as_str()) {
        ("macos"  , "x86_64")  => "src/macos/raw.rs",
        ("linux"  , "x86_64")  => "src/linux/raw.rs",
        ("linux"  , "aarch64") => "src/linux/raw_aarch64.rs",
        ("windows", "x86_64")  => "src/windows/raw.rs",
        ("windows", "x86"   )  => "src/windows/raw_32.rs",
        _ => panic!(
                "target `{}` haven't binding file, generate it with `cargo build --features gen`",
                env::var("TARGET").unwrap()
            )
    };

    fs::copy(bindings, out_file).expect("fail to copy bindings");
}

fn main() {
    env_logger::init();

    let libhs = find_hyperscan();

    let out_dir = env::var("OUT_DIR").unwrap();
    let out_file = Path::new(&out_dir).join("raw.rs");

    generate_binding(libhs.include_paths[0].to_str().unwrap(), &out_file);

    for lib in libhs.libs {
        if lib.contains("hs") {
            println!("cargo:rustc-link-lib=static={}", lib);
        }
    }

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    if target_os == "macos" || target_os == "freebsd" {
        println!("cargo:rustc-link-lib=dylib=c++");
    } else if target_os != "windows" {
        println!("cargo:rustc-link-lib=dylib=stdc++");
        println!("cargo:rustc-link-lib=dylib=gcc");
    }

    for link_path in libhs.link_paths {
        println!("cargo:rustc-link-search=native={}", link_path.to_str().unwrap());
    }
}
