use crate::helpers::adb_command;
use chrono;

fn get_timestamped_filename() -> String {
    let timestamp = chrono::Local::now().format("%Y-%m-%d_%H-%M-%S");
    format!("screenshot{}.png", timestamp)
}

pub fn screenshot(filename: Option<String>) {
    let filepath = filename
        .or_else(|| Some(get_timestamped_filename()))
        .unwrap();
    adb_command(&["shell", "screencap -a -p", ">", &filepath], false);
}

pub fn screenrecord() {
    adb_command(
        &["shell", "settings", "put", "system", "show_touches", "1"],
        true,
    );
    adb_command(
        &[
            "shell",
            "screenrecord",
            "--verbose",
            "/sdcard/screenrecord.mp4",
        ],
        true,
    );
    println!("Screenrecord finished");
    adb_command(
        &[
            "pull",
            "/sdcard/screenrecord.mp4",
            &get_timestamped_filename(),
        ],
        true,
    );
    adb_command(
        &["shell", "settings", "put", "system", "show_touches", "0"],
        true,
    );
}
