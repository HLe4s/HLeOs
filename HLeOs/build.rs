extern crate cc;

fn main() {
    println!("cargo:rerun-if-changed=c_src/dummy_c.c");
    cc::Build::new()
        .file("c_src/dummy_c.c")
        .compile("dummy_c.o");
}
