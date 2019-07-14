extern crate cc;

fn main() {
    cc::Build::new()
        .file("wrappers/target.c")
        .compile("llvm_target");
}
