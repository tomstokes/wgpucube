use crate::XtaskCommand;
use clap::ArgMatches;
use xshell::Shell;

pub(crate) struct RunWasm {}

impl XtaskCommand for RunWasm {
    fn run(shell: &Shell, _matches: &ArgMatches) {
        // TODO: Return exit codes on error
        xshell::cmd!(shell, "echo Run wasm is a work in progress")
            .run()
            .unwrap();
    }
}
