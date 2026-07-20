fn main() {
    // embed the compiler version for the report's Environment block
    let version = std::process::Command::new("rustc")
        .arg("--version")
        .output()
        .map(|out| String::from_utf8_lossy(&out.stdout).trim().to_string())
        .unwrap_or_else(|_| "rustc (unknown)".to_string());
    println!("cargo:rustc-env=BENCH_RUSTC_VERSION={version}");
}
