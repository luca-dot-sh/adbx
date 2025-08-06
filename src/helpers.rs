use chrono::format;
use clap::builder::Str;

use crate::global_flags::VERBOSE;
use crate::printer::{print_dbg, println_debug};
use std::io::{self, Write};
use std::process::{Command, Stdio};

pub fn adb_command(args: &[&str], capture: bool) -> Result<String, std::io::Error> {
    let output_mode = if capture || VERBOSE.load(std::sync::atomic::Ordering::Relaxed) {
        Stdio::piped()
    } else {
        Stdio::inherit()
    };

    println_debug(&format!("Running command: adb {}\n", args.join(" ")));
    let output = Command::new("adb")
        .args(args) // Pass all arguments at once
        .stdout(output_mode) // Print stdout directly
        .stderr(Stdio::inherit()) // Print stderr directly
        .output()?;

    if !output.status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Command failed with status: {}", output.status),
        ));
    }

    let output_str = String::from_utf8(output.stdout)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    println_debug(&format!("Command output: {}", output_str));
    return Ok(output_str);
}

pub fn get_package_uids(package_name: &str) -> String {
    return adb_command(
        &[
            "shell",
            &format!("pm list package -U {} | sed 's/.*uid://'", package_name),
        ],
        true,
    )
    .expect("failed to get package uids")
    .lines()
    .collect::<Vec<_>>()
    .join(",");
}

pub fn rr() {
    adb_root();
    adb_command(&["wait-for-device"], false);
    adb_command(&["remount"], false);
}

pub fn rrr() {
    rr();
    adb_command(&["reboot"], false);
}

pub fn get_package_path(package_name: &str) -> String {
    adb_command(&["shell", &format!("pm path {}", package_name)], true)
        .map(|s| s.trim().strip_prefix("package:").unwrap().to_string())
        .expect("failed to get package path")
}

pub fn adb_root() {
    adb_command(&["root"], true);
}
