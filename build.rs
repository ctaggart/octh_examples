fn main () {
    // println!("cargo:rustc-link-search={}", "C:\\Octave\\Octave-4.2.1\\lib\\octave\\4.2.1");
    // println!("cargo:rustc-link-search={}", "C:\\Octave\\Octave-4.2.1\\lib");
    // println!("cargo:rustc-link-search={}", "C:\\Octave\\Octave-4.2.1\\lib\\gcc\\x86_64-w64-mingw32\\4.9.4")
    // println!("cargo:rustc-link-search={}", "C:\\Octave\\Octave-4.2.1\\lib64\\gcc\\x86_64-w64-mingw32\\4.9.4");
    
    println!("cargo:rustc-link-search={}", "/home/cameron/octave-build/liboctave/.libs");
    println!("cargo:rustc-link-search={}", "/home/cameron/octave-build/libinterp/.libs");
    
    println!("cargo:rustc-link-lib={}", "octave");
    println!("cargo:rustc-link-lib={}", "octinterp");
    // println!("cargo:rustc-link-lib={}", "octhc");
    // println!("cargo:rustc-link-lib={}", "pthread");

    // println!("cargo:rustc-link-lib={}", "freetype");
    // println!("cargo:rustc-link-lib={}", "hdf5");
    // println!("cargo:rustc-link-lib={}", "GraphicsMagick++");
    // println!("cargo:rustc-link-lib={}", "GraphicsMagick");
    // println!("cargo:rustc-link-lib={}", "z");
    // println!("cargo:rustc-link-lib={}", "fftw3");
    // println!("cargo:rustc-link-lib={}", "fftw3f");
    // println!("cargo:rustc-link-lib={}", "opengl32");
    // println!("cargo:rustc-link-lib={}", "glu32");
    // println!("cargo:rustc-link-lib={}", "fontconfig");
    // println!("cargo:rustc-link-lib={}", "freetype");
    // println!("cargo:rustc-link-lib={}", "gl2ps");
    // println!("cargo:rustc-link-lib={}", "lapack");
    // println!("cargo:rustc-link-lib={}", "curl");
    // println!("cargo:rustc-link-lib={}", "cholmod");
    // println!("cargo:rustc-link-lib={}", "umfpack");
    // println!("cargo:rustc-link-lib={}", "amd");
    // println!("cargo:rustc-link-lib={}", "camd");
    // println!("cargo:rustc-link-lib={}", "colamd");
    // println!("cargo:rustc-link-lib={}", "ccolamd");
    // println!("cargo:rustc-link-lib={}", "cxsparse");
    // println!("cargo:rustc-link-lib={}", "suitesparseconfig");
    // println!("cargo:rustc-link-lib={}", "arpack");
    // println!("cargo:rustc-link-lib={}", "qrupdate");
    // println!("cargo:rustc-link-lib={}", "blas");
    // println!("cargo:rustc-link-lib={}", "readline");
    // println!("cargo:rustc-link-lib={}", "termcap");
    // println!("cargo:rustc-link-lib={}", "pcre");
    // println!("cargo:rustc-link-lib={}", "m");
    // println!("cargo:rustc-link-lib={}", "gfortran");
    // println!("cargo:rustc-link-lib={}", "moldname");
    // println!("cargo:rustc-link-lib={}", "quadmath");

}