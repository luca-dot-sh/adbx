use crate::{adb_command, helpers::get_package_path};

pub fn modifications() {
    let modified_packages = adb_command(&["shell", "pm list packages -f -3"], true);
    match modified_packages {
        Ok(output) => {
            let modified_packages = output.lines().collect::<Vec<_>>();
            match modified_packages.len() {
                0 => println!("No packages were modified :)"),
                _ => {
                    println!("The following packages were modified:");
                    modified_packages
                        .iter()
                        .for_each(|package| println!("{}", package));
                }
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}

pub fn version(package_name: &str) {
    println!("Installed Versions: ");
    let output = adb_command(
        &[
            "shell",
            "dumpsys",
            "package",
            package_name,
            "| grep versionName",
        ],
        true,
    );
    println!(
        "{}",
        output
            .unwrap()
            .lines()
            .map(|line| line.trim())
            .collect::<Vec<_>>()
            .join("\n")
    );
}

pub fn pull(package_name: &str) {
    let path_on_device = get_package_path(package_name);
    adb_command(&["pull", &path_on_device], false);
}

pub fn install(path_to_apk: &str, package_name: &str) {
    let path_on_device = get_package_path(package_name);
    adb_command(&["push", &path_to_apk, &path_on_device], false);
}
