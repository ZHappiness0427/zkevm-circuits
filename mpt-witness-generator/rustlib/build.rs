use std::{
    env,
    io::{self, Write},
};

fn main() {
    let lib_name = "mpt-witness-generator";
    let out_dir = env::var("OUT_DIR").unwrap();

    // Build
    if let Err(e) = gobuild::Build::new()
        .file("../witness_gen_wrapper.go")
        .try_compile(lib_name)
    {
        // The error type is private so have to check the error string
        if format!("{}", e).starts_with("Failed to find tool.") {
            fail(
                " Failed to find Go. Please install Go 1.16 or later \
                following the instructions at https://golang.org/doc/install.
                On linux it is also likely available as a package."
                    .to_string(),
            );
        } else {
            fail(format!("{}", e));
        }
    }

    // Files the lib depends on that should recompile the lib
    let dep_files = glob::glob("../**/*.go").unwrap().filter_map(|v| v.ok());

    for file in dep_files {
        println!("cargo:rerun-if-changed={}", file.to_str().unwrap());
    }

    // Link
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static={}", lib_name);
}

fn fail(message: String) {
    let _ = writeln!(
        io::stderr(),
        "\n\nError while building mpt-witness-generator: {}\n\n",
        message
    );
    std::process::exit(1);
}
