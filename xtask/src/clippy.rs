use crate::XtaskCommand;
use clap::{ArgMatches, Command};
use xshell::Shell;

pub(crate) struct Clippy {}

impl XtaskCommand for Clippy {
    fn command() -> Command {
        Command::new("clippy").about("Run Clippy for all combinations of targets and features ")
    }

    fn run(shell: &Shell, _matches: &ArgMatches) {
        // TODO: Return exit codes on error
        // TODO: Support release and debug targets
        // TODO: Check that correct toolchains are installed?

        let default_args: &[&str] = &["--bins"];
        let android_args: &[&str] = &["--lib", "--bin", "xtask"];

        let targets = [
            ("aarch64-apple-darwin", default_args),
            ("aarch64-apple-ios", default_args),
            ("aarch64-linux-android", android_args),
            ("x86_64-pc-windows-msvc", default_args),
            ("wasm32-unknown-unknown", default_args),
            ("x86_64-unknown-linux-gnu", default_args),
        ];
        let features = ["", "egui"];

        for (target, clippy_targets) in targets {
            for feature in features {
                xshell::cmd!(
                    shell,
                    "cargo clippy --target {target} --features {feature} --workspace {clippy_targets...} -- -D warnings"
                )
                    .run()
                    .unwrap();
            }
        }
    }
}
