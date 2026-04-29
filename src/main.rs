//! Prints the detected AI coding agent name and exits 0.
//! Exits 1 if no agent is detected.

use agent_detector::agent_name;

/// Entry point for the `whichagent` CLI binary.
///
/// Prints the detected agent name to stdout and exits 0, or exits 1 if no
/// agent is detected.
fn main() {
    if let Some(name) = agent_name() {
        println!("{name}");
    } else {
        std::process::exit(1);
    }
}
