use sysinfo::{Pid, ProcessesToUpdate, System};

use crate::agents::PARENT_PROCESS_NAMES;

pub fn find_agent_in_parent_tree() -> Option<String> {
    let mut system = System::new();
    system.refresh_processes(ProcessesToUpdate::All, true);

    let current_pid = Pid::from_u32(std::process::id());
    let mut pid = Some(current_pid);

    while let Some(p) = pid {
        let proc = system.process(p)?;

        let name = proc.name();

        for &(process_name, agent_name) in PARENT_PROCESS_NAMES {
            if name == process_name {
                return Some(agent_name.to_string());
            }
        }

        pid = proc.parent();
    }

    None
}
