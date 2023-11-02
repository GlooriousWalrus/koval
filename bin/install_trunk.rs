#!/usr/bin/env -S cargo +nightly -Zscript

//! ```cargo
//! [package]
//! name = "install_trunk"
//! version = "0.1.0"
//!
////! [dependencies]
////! clap = { version = "4.2", features = ["derive"] }
//! ```

use std::env;
use std::process::Command;

fn main() {
    // Default trunk version 0.17.5
    let mut install_version = "0.17.5".to_string();
    let target_version = match env::var("TRUNK_VERSION") {
        Ok(version) => {
            install_version = version;
            format!("trunk {}", install_version)
        }
        Err(_e) => format!("trunk {}", install_version),
    };

    println!("{}", target_version);

    let trunk_is_installed = has_cargo_installed();
    if trunk_is_installed {
        println!("Trunk is installed.");
        // Get the current trunk version
        let output = Command::new("sh")
            .arg("-c")
            .arg("trunk --version")
            .output()
            .expect("failed to execute process");

        let version = String::from_utf8_lossy(&output.stdout);
        let version = version
            .strip_suffix("\r\n")
            .or(version.strip_suffix("\n"))
            .unwrap_or(&version);

        // let status_code = match output.status.code() {
        //     Some(code) => code,
        //     None => -1,
        // };

        // println!("success: {}", output.status.success());
        // println!("status: {}", output.status);
        // println!("status_code: {}", status_code.to_string());
        // println!("stdout: {}", version);
        // println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

        if target_version == version {
            // Everything is fine and nothing is needed.
            println!("The version requested matches installed.");
        } else {
            // Force the install of the new version trunk
            println!(
                "Overwriting trunk with version:{}. Please be patient!",
                install_version.clone()
            );

            let cmd = format!(
                "cargo install --locked trunk --version={} --force",
                install_version
            );
            install(&cmd);
        }
    } else {
        println!("Trunk is NOT installed.");
        println!(
            "Installing trunk version:{}. Please be patient!",
            install_version.clone()
        );

        let cmd = format!("cargo install --locked trunk --version={}", install_version);
        install(&cmd);
    }
}

fn has_cargo_installed() -> bool {
    let output = Command::new("sh")
        .arg("-c")
        .arg("ls -f ~/.cargo/bin/trunk")
        .output()
        .expect("failed to execute process");

    let status_code = match output.status.code() {
        Some(code) => code,
        None => -1,
    };
    status_code == 0
}

fn install(cmd: &str) -> bool {
    let output = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("failed to execute process");

    let status_code = match output.status.code() {
        Some(code) => code,
        None => -1,
    };
    status_code == 0
}
