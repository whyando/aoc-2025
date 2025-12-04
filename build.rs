fn main() {
    // Rebuild if the assembly source changes.
    println!("cargo:rerun-if-changed=src/day01_asm.s");

    // Compile the pure assembly implementation into a static library.
    cc::Build::new()
        .file("src/day01_asm.s")
        .compile("day01_asm");
}
