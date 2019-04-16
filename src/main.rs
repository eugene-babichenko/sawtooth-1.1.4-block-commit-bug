mod engine;

use crate::engine::*;
use docopt::Docopt;

const USAGE: &'static str = "
A dummy Sawtooth consensus engine to reproduce some bugs.

Usage: remme-consensus [options]

Options:
  -h --help                  Show this screen.
  -c --connect <endpoint>    Set a consensus CSDK endpoint [default: tcp://127.0.0.1:5005].
";

fn main() {
    let args = Docopt::new(USAGE)
        .and_then(|dopt| dopt.parse())
        .unwrap_or_else(|e| e.exit());
    let endpoint = args.get_str("--connect");

    let (driver, _) = sawtooth_sdk::consensus::zmq_driver::ZmqDriver::new();
    driver
        // You can change the type of the bug here
        .start(endpoint, ConsensusEngine::new(BugType::SimultaneousCommits))
        .unwrap_or_else(|err| {
            eprintln!("Error while running the consensus engine: {}", err);
            std::process::exit(1);
        });
}
