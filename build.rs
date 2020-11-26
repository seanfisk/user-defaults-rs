use std::env;
use std::path::{Path, PathBuf};

use bindgen::Bindings;

fn main() {
    // If we don't set this, bindgen will emit a warning. Things still seem to
    // work, but we may as well tell bindgen where llvm-config is. It doesn't
    // work to just set this var for rustc.
    env::set_var("LLVM_CONFIG_PATH", "/usr/local/opt/llvm/bin/llvm-config"); // Homebrew

    let macos_min_version = "10.10";
    let macos_min_version_flag = "-mmacosx-version-min=10.10";
    println!(
        "cargo:rustc-env=MACOSX_DEPLOYMENT_TARGET={}",
        macos_min_version
    );

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let ffi_dir = PathBuf::from("src/ffi");

    let process_input_file = |file_name: &str| -> String {
        let path = ffi_dir.join(file_name);
        let path_str = path.to_str().expect("Converting path to string");
        rerun_if_changed(&path);
        path_str.to_owned()
    };

    let write_bindings = |bindings: Bindings, module_name: &str| {
        let path = out_path.join(format!("{}.rs", module_name));
        bindings.write_to_file(path).expect("Writing bindings");
    };

    // User defaults
    {
        let bindings = generate(
            bindgen::Builder::default()
                .header(process_input_file("user_defaults.h"))
                .whitelist_type("user_defaults_.+")
                .whitelist_function("user_defaults_.*"),
        );

        write_bindings(bindings, "user_defaults");

        cc::Build::new()
            .file(process_input_file("user_defaults.m"))
            .flag("-fmodules")
            .flag(macos_min_version_flag)
            .compile("user_defaults");
    }
}

fn rerun_if_changed(path: &Path) {
    // Tell cargo to invalidate the built crate whenever the path changes
    println!(
        "cargo:rerun-if-changed={}",
        path.to_str().expect("Converting path to string")
    );
}

fn generate(builder: bindgen::Builder) -> bindgen::Bindings {
    builder.generate().expect("Generating bindings")
}
