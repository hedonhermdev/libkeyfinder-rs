fn main() -> Result<(), Box<dyn std::error::Error>> {

    let fftw3_dst = pkg_config::Config::new().atleast_version("3.3.10").probe("fftw3")?;

    let keyfinder_dst = cmake::Config::new("libkeyfinder").configure_arg("-DBUILD_SHARED_LIBS=OFF").configure_arg("-DBUILD_TESTING=OFF").always_configure(true).build().join("build");

    println!("\n\n\n\nHEREJREKREKRJEJK");
    dbg!(&keyfinder_dst);
    println!("\n\n\n\nHEREJREKREKRJEJK");

    println!("cargo:rustc-link-search={}", keyfinder_dst.display());
    println!("cargo:rustc-link-search={}", fftw3_dst.link_paths[0].display());
    println!("cargo:rustc-link-lib=static=fftw3");
    println!("cargo:rustc-link-lib=static=keyfinder");

    cxx_build::bridge("src/lib.rs")
        .file("src/key.cc")
        .flag_if_supported("-std=c++14")
        .flag_if_supported("-s")
        .compile("cxxbridge-demo");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/key.cc");
    println!("cargo:rerun-if-changed=include/key.hpp");

    Ok(())
}
