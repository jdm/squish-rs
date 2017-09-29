extern crate gcc;

fn main() {
    gcc::Config::new()
        .file("src/squish.cpp")
        .include("squish_sys")
        .compile("libsquish_c.a");
    println!("cargo:rustc-link-lib=static=squish");
    //gcc::compile_library("libsquish_c.a", &["src/squish.cpp"]);
}
