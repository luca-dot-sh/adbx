mod vhal;
mod helpers;
mod packages;
mod logging;
mod global_flags;
mod printer;
mod tracing;

use clap::Parser;
use clap::Subcommand;
use core::str;
use helpers::{adb_root,adb_command};
use std::sync::atomic::Ordering::Relaxed;

#[derive(Parser)]
struct Cli {
    #[clap(long, global = true, help = "Enable verbose output (if you want to see implementation)", short = 'v')]
    verbose: bool,

    #[clap(subcommand)]
    command: AdbXCommand,
}

#[derive(Subcommand, Debug)]
enum AdbXCommand {
    /// Installs a system app to the device
    Install { path_to_apk: String },
    /// Print the logs of a package
    Applog { package_name: String },
    /// Check if any packages were modified
    Modifications {},
    /// Simulate driving at 30 km/h
    StartDriving {},
    /// Simulate driving at 0 km/h
    StopDriving {},
    /// Print the version of a package
    Version { package_name: String },
    /// Record the screen and save it to a file
    Screenrecord {},
}

fn main() {
    adb_root();
    let args = Cli::parse();
    let cmd = args.command;
    global_flags::VERBOSE.store(args.verbose,Relaxed);
    match cmd {
        AdbXCommand::Applog { package_name } => logging::app_log(&package_name),
        AdbXCommand::Install { path_to_apk } => packages::install(&path_to_apk),
        AdbXCommand::Modifications {} => packages::modifications(),
        AdbXCommand::StartDriving {} => vhal::start_driving(), 
        AdbXCommand::StopDriving {} => vhal::stop_driving(),
        AdbXCommand::Version { package_name } => packages::version(&package_name),
        AdbXCommand::Screenrecord {} => tracing::screenrecord(),
    }
}





