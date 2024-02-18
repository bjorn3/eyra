fn main() {
    // Pass -nostartfiles to the linker.
    println!("cargo:rustc-link-arg=-nostartfiles");
    println!("cargo:rustc-link-arg=/home/bjorn/Projects/rust_musl_port/musl/obj/ldso/dynlink.lo");
}
