fn main() {
    let mut build = cc::Build::new();
    build.file("src/asm.S");
    build.compile("asm");
}
