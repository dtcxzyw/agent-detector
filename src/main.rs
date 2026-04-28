use agent_detector::{detect, is_agent};

fn main() {
    if is_agent() {
        let info = detect().unwrap();
        println!("{}", info.name);
    } else {
        std::process::exit(1);
    }
}
