use std::process::Command;

fn main() {
    tauri_build::build();
    let output = Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()
        .expect("cant get git commit info");

    let git_commit = String::from_utf8_lossy(&output.stdout).trim().to_string();

    println!("cargo:rustc-env=git_commit={}", git_commit);

    let build_time = chrono::Utc::now().to_rfc3339();

    println!("cargo:rustc-env=build_time={}", build_time);
}
