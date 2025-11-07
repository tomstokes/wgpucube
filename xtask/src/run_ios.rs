use crate::XtaskCommand;
use clap::ArgMatches;
use xshell::Shell;

pub(crate) struct RunIos {}

impl XtaskCommand for RunIos {
    fn run(shell: &Shell, _matches: &ArgMatches) {
        // TODO: Return exit codes on error
        // TODO: Support release and debug targets
        // TODO: Check that necessary tools are installed
        // TODO: Support running on simulator or real device
        // TODO: Add proper package metadata for cargo-bundle to use

        let iphone_model = "iPhone 16e";

        xshell::cmd!(
            shell,
            "cargo bundle --bin wgpucube --package wgpucube --target aarch64-apple-ios-sim"
        )
        .run()
        .unwrap();
        xshell::cmd!(shell, "xcrun simctl boot {iphone_model}")
            .run()
            .unwrap();
        xshell::cmd!(shell, "open -a Simulator").run().unwrap();
        xshell::cmd!(
            shell,
            "xcrun simctl install booted target/aarch64-apple-ios-sim/debug/bundle/ios/wgpucube.app"
        )
        .run()
        .unwrap();
        xshell::cmd!(
            shell,
            "xcrun simctl launch --console booted wgpucube.wgpucube"
        )
        .run()
        .unwrap();
    }
}
