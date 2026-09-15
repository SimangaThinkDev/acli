pub enum CommandPolicy {
    Allowed,
    Restricted,
    Denied,
}

const RESTRICTED_COMMANDS: &[&str] = &[
    "rm",
    "rmdir",
    "mv",
    "dd",
    "mkfs",
    "chmod",
    "chown",
    "kill",
    "pkill",
    "systemctl",
];

const DENIED_COMMANDS: &[&str] = &[
    "shutdown",
    "reboot",
    "poweroff",
];

pub fn check_command(command: &str) -> CommandPolicy {
    let program = command.split_whitespace().next().unwrap_or("");

    if DENIED_COMMANDS.contains(&program) {
        return CommandPolicy::Denied;
    }

    if RESTRICTED_COMMANDS.contains(&program) {
        return CommandPolicy::Restricted;
    }

    CommandPolicy::Allowed
}
