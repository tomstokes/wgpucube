mod run_wasm;

use run_wasm::RunWasm;
use std::path::Path;

use clap::{ArgMatches, Command};
use xshell::Shell;

pub trait XtaskCommand {
    // TODO: This should return an exit code
    fn run(shell: &Shell, matches: &ArgMatches);
}

fn main() {
    let cmd = clap::Command::new("xtask")
        .bin_name("xtask")
        .subcommand_required(true)
        .subcommand(Command::new("run-wasm").about("Build and serve wasm version"));
    let matches = cmd.get_matches();
    let shell = Shell::new().unwrap();
    let workspace_root_path = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    shell.change_dir(workspace_root_path);
    match matches.subcommand() {
        Some(("run-wasm", sub_matches)) => RunWasm::run(&shell, sub_matches),
        _ => unreachable!(),
    }
}
