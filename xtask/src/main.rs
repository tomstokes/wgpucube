mod run_ios;
mod run_wasm;

use clap::{ArgMatches, Command};
use run_ios::RunIos;
use run_wasm::RunWasm;
use std::path::Path;
use xshell::Shell;

pub trait XtaskCommand {
    fn command() -> Command;

    // TODO: This should return an exit code
    fn run(shell: &Shell, matches: &ArgMatches);
}

fn main() {
    let cmd = Command::new("xtask")
        .bin_name("xtask")
        .subcommand_required(true)
        .subcommand(RunIos::command())
        .subcommand(RunWasm::command());
    let matches = cmd.get_matches();
    let shell = Shell::new().unwrap();
    let workspace_root_path = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    shell.change_dir(workspace_root_path);
    match matches.subcommand() {
        Some(("run-wasm", sub_matches)) => RunWasm::run(&shell, sub_matches),
        Some(("run-ios", sub_matches)) => RunIos::run(&shell, sub_matches),
        _ => unreachable!(),
    }
}
