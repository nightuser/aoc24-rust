fn main() {
    // See https://github.com/blas-lapack-rs/accelerate-src
    println!("cargo:rustc-link-lib=framework=Accelerate");
}
