use std::env;
use std::process::{Command, Stdio};

fn main() {
    let result = Command::new("make")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .unwrap();
    assert!(result.success());
    let out_dir = env::var("OUT_DIR").unwrap();
    let result = Command::new("cp")
        .arg("libsquish.a")
        .arg(out_dir.clone())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .unwrap();
    assert!(result.success());
    //println!("cargo:rustc-link-search=native=squish_sys");
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=squish");
    //println!("cargo:include=.");
    println!("cargo:outdir={}", out_dir);
}
