use agent_detector::agent_name;

fn main() {
    if let Some(name) = agent_name() {
        println!("{name}");
    } else {
        std::process::exit(1);
    }
}
