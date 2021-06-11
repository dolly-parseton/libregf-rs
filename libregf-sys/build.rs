use std::{env, path::PathBuf, process::Command};

fn main() {
    println!("cargo:rerun-if-changed=./build.rs");

    let dst = PathBuf::from(env::var("OUT_DIR").unwrap());
    // Clone Source
    if !PathBuf::from("./libregf").exists() {
        Command::new("git")
            .arg("clone")
            .arg("https://github.com/libyal/libregf.git")
            .output()
            .expect("failed to run clone libregf during build process.");
    }

    // Clean Source
    Command::new("./synclibs.sh")
        .current_dir("./libregf")
        .arg("distclean")
        .output()
        .expect("failed to run ./synclibs.sh during build process.");
    // Run ./autogen.sh
    Command::new("./autogen.sh")
        .current_dir("./libregf")
        .output()
        .expect("an error occured while running ./autogen.sh during build process");
    // Run configure with the dst variable
    Command::new("./configure")
        .arg("--enable-shared=no")
        .arg("--enable-static-executables=no")
        .arg(format!("--prefix={}", dst.display()))
        .current_dir("./libregf")
        .output()
        .expect("an error occured while running ./configure during build process");
    // Run make to install the lib to the dst directory
    Command::new("make")
        .arg("install")
        .current_dir("./libregf")
        .output()
        .expect("an error occured while running 'make install' during build process");
    // Configure rustc
    println!("cargo:root={}", dst.display());
    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=regf");
    // Generate bindings
    let bindings = bindgen::Builder::default()
        .detect_include_paths(true)
        // .enable_cxx_namespaces()
        .clang_arg("-I")
        .clang_arg("./libregf/include")
        .clang_arg("-x")
        .clang_arg("c++")
        .header("./libregf/include/libregf.h")
        .rustified_enum("LIBREGF_ACCESS_FLAGS")
        .rustified_enum("LIBREGF_FILE_TYPES")
        .rustified_enum("LIBREGF_CODEPAGES")
        .rustified_enum("LIBREGF_ARGUMENT_ERROR")
        .rustified_enum("LIBREGF_CONVERSION_ERROR")
        .rustified_enum("LIBREGF_COMPRESSION_ERROR")
        .rustified_enum("LIBREGF_VALUE_TYPES")
        .rustified_enum("LIBREGF_ERROR_DOMAINS")
        .rustified_enum("LIBREGF_IO_ERROR")
        .rustified_enum("LIBREGF_INPUT_ERROR")
        .rustified_enum("LIBREGF_MEMORY_ERROR")
        .rustified_enum("LIBREGF_OUTPUT_ERROR")
        .rustified_enum("LIBREGF_RUNTIME_ERROR")
        .generate()
        .expect("an error occurred while generating bindings");

    // Write the bindings to dst
    bindings
        .write_to_file(dst.join("bindings.rs"))
        .expect("an error occurred while writing bindings");
}
