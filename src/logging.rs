use crate::helpers::{adb_command, get_package_uids};

pub fn app_log(package_name: &str) {
    let package_uid_str = get_package_uids(package_name);
    adb_command(&["logcat", "--uid", &package_uid_str], false);
}