use clap::Command;

fn main() {
    let cmd = clap::Command::new("xtask")
        .bin_name("xtask")
        .subcommand_required(true)
        .subcommand(Command::new("run-wasm").about("Build and serve wasm version"));
    let matches = cmd.get_matches();
    match matches.subcommand() {
        Some(("run-wasm", _)) => {
            println!("run-wasm not implemented yet");
        }
        _ => unreachable!(),
    }
}
