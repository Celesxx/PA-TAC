fn main() {
    let mkl_lib_path = "C:\\Program Files (x86)\\Intel\\oneAPI\\mkl\\latest\\lib";
    println!("cargo:rustc-link-search=native={}", mkl_lib_path);
    println!("cargo:rustc-link-lib=dylib=mkl_intel_lp64");
    println!("cargo:rustc-link-lib=dylib=mkl_sequential");
    println!("cargo:rustc-link-lib=dylib=mkl_core");

    // Print out some debugging information
    for (key, value) in std::env::vars() {
        println!("cargo:warning={}={}", key, value);
    }
    println!("cargo:warning=MKL library path: {}", mkl_lib_path);
}
