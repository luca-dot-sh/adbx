use crate::adb_command;

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
    let output =adb_command(&["shell", "dumpsys", "package", package_name, "| grep versionName"], true);
    println!("{}", output.unwrap().lines().map(|line| line.trim()).collect::<Vec<_>>().join("\n"));
}

pub fn install(path: &str) {
    adb_command(&["root"], false)
        .and(adb_command(&["remount"], false))
        .and(adb_command(&["reboot"], false))
        .and(adb_command(&["wait-for-device"], false))
        .and(adb_command(&["root"], false))
        .and(adb_command(&["remount"], false))
        .expect_err("install failed");
}