use std::process::Command;

fn main() {
    Command::new("./ior/bootstrap").status().unwrap();
    // Note that there are a number of downsides to this approach, the comments
    // below detail how to improve the portability of these commands.
    Command::new("./ior/configure")
        .args(&["CFLAGS=\"-g -O0 -fPIC\""])
        .status()
        .unwrap();
    Command::new("make").args(&["install"]).status().unwrap();
    println!("cargo:rustc-link-lib=aiori");
    println!("cargo:rustc-link-lib=mpi");
}
