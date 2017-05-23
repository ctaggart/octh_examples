fn main () {
    println!("cargo:rustc-link-search={}", "C:\\Octave\\OCTAVE~1.1\\lib\\octave\\4.2.1");
    println!("cargo:rustc-link-lib={}", "octave");
    println!("cargo:rustc-link-lib={}", "octinterp");
    // println!("cargo:rustc-link-lib={}", "octhc");
    println!("cargo:rustc-link-lib={}", "pthread");
}