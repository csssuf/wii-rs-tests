use std::env;

fn main() {
    let libogc = env::var("LIBOGC").expect("LIBOGC unset");
    let dkppc = env::var("DEVKITPPC").expect("DEVKITPPC unset");

    println!("cargo:rustc-link-search={}/lib/wii", libogc);
    println!("cargo:rustc-link-search={}/powerpc-eabi/lib", dkppc);

    let libraries = ["wiiuse", "bte", "ogc"];
    for lib in &libraries {
        println!("cargo:rustc-link-lib=static={}", lib);
    }
}
