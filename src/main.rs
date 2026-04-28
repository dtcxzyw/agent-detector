//! Prints the detected AI coding agent name and exits 0.
//! Exits 1 if no agent is detected.

use agent_detector::agent_name;

fn main() {
    if let Some(name) = agent_name() {
        println!("{name}");
    } else {
        std::process::exit(1);
    }
}
