use std::{env, path::Path, process::Command};

fn main() {
    println!("cargo:rerun-if-changed=./ior");
    let root = Path::new("./ior");
    env::set_current_dir(&root).is_ok();
    Command::new("git").args(&["submodule","init"]).status().unwrap();
    Command::new("git").args(&["submodule","update"]).status().unwrap();
    Command::new("./bootstrap").status().unwrap();
    // Note that there are a number of downsides to this approach, the comments
    // below detail how to improve the portability of these commands.
    Command::new("./configure")
        .args(&["CFLAGS=\"-g -O0 -fPIC\""])
        .status()
        .unwrap();
    Command::new("make").args(&["install"]).status().unwrap();
    println!("cargo:rustc-link-lib=aiori");
    println!("cargo:rustc-link-lib=mpi");
}
