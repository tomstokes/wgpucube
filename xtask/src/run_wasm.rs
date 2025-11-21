use crate::XtaskCommand;
use clap::{ArgMatches, Command};
use xshell::Shell;

pub(crate) struct RunWasm {}

impl XtaskCommand for RunWasm {
    fn command() -> Command {
        Command::new("run-wasm").about("Build and serve wasm version")
    }

    fn run(shell: &Shell, _matches: &ArgMatches) {
        // TODO: Return exit codes on error
        // TODO: Support release and debug targets

        shell.change_dir("web");
        xshell::cmd!(shell, "trunk serve").run().unwrap();
    }
}
