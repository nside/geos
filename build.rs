fn main() {
    // Call geos-config to get the path of the static library
    let output = std::process::Command::new("geos-config")
        .arg("--libs")
        .output()
        .expect("Failed to execute geos-config");

    // Parse the output of geos-config to get the path of the static library
    let lib_path = String::from_utf8(output.stdout)
        .expect("Failed to parse output of geos-config")
        .trim()
        .split(' ')
        .find(|s| s.starts_with("-L"))
        .map(|s| s[2..].to_owned())
        .expect("Failed to find library path in output of geos-config");

    // Pass the path of the static library to Cargo
    println!("cargo:rustc-flags=-L {}", lib_path);
}
