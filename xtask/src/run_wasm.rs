use crate::XtaskCommand;
use clap::ArgMatches;
use xshell::Shell;

pub(crate) struct RunWasm {}

impl XtaskCommand for RunWasm {
    fn run(shell: &Shell, _matches: &ArgMatches) {
        // TODO: Return exit codes on error
        // TODO: Support release and debug targets
        xshell::cmd!(shell, "cargo build --target wasm32-unknown-unknown")
            .run()
            .unwrap();
        xshell::cmd!(shell, "wasm-bindgen target/wasm32-unknown-unknown/debug/wgpucube.wasm --target web --out-dir target/generated --out-name wgpucube")
            .run()
            .unwrap();
        shell
            .copy_file("./web/index.html", "./target/generated")
            .unwrap();
        xshell::cmd!(shell, "simple-http-server target/generated -c html,js,wasm --index --nocache --coep --coop --ip 127.0.0.1 --port 8000").run().unwrap();
    }
}
