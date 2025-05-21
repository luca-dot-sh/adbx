use crate::Cli;
use clap::CommandFactory;
use clap_complete::generate;
use clap_complete::Shell::Bash;

pub fn generate_completion_script() {
    let mut buffer = Vec::new();
    generate(Bash, &mut Cli::command(), "adbx", &mut buffer);
    let mut completion_script = String::from_utf8(buffer).unwrap();
    completion_script = completion_script.replace(
        "<PACKAGE_NAME>",
        "$(adb shell pm list packages | sed 's/^package://' | tr '\n' ' ' | sed 's/ $//')",
    );
    println!("{}", completion_script);
}
