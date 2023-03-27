use std::{
    collections::VecDeque,
    io::{self, BufRead},
};

use regex::Regex;

pub struct Config {
    _excluded_windows: Vec<Regex>,
    managed_windows: Vec<Regex>,
    workspaces_monitors: Vec<String>,
}
impl Config {
    pub fn _is_excluded(&self, window_title: &str) -> bool {
        for regex in self._excluded_windows.iter() {
            if regex.is_match(&window_title) {
                return true;
            }
        }

        false
    }

    pub fn is_managed(&self, window_title: &str) -> bool {
        for regex in self.managed_windows.iter() {
            if regex.is_match(&window_title) {
                return true;
            }
        }

        false
    }
}

enum Command {
    RuleExclude(Regex),
    RuleManaged(Regex),
    Workspace(usize, String),
}

pub struct ConfigBuilder {
    commands: Vec<Command>,
}
impl ConfigBuilder {
    pub fn new(config_path: &str) -> Self {
        let commands: Vec<Command> = ConfigBuilder::parse_commands(config_path);

        Self { commands }
    }

    fn parse_commands(config_path: &str) -> Vec<Command> {
        let mut commands = Vec::new();

        let config_file =
            std::fs::File::open(config_path).expect("Failed to open the configuration file");

        let lines = io::BufReader::new(config_file).lines();
        for line in lines {
            if let Ok(line) = line {
                if line.is_empty() {
                    continue;
                }

                let mut splitted_line: VecDeque<&str> = line.split(" ").collect();

                if let Some(command) = splitted_line.pop_front() {
                    match command {
                        "workspace" => {
                            if let Some(workspace_index) = splitted_line.pop_front() {
                                let monitor_name = Vec::from(splitted_line).join(" ");
                                if let Ok(workspace_index) = str::parse::<usize>(workspace_index) {
                                    commands
                                        .push(Command::Workspace(workspace_index, monitor_name));
                                }
                            }
                        }
                        "rule" => {
                            if let Some(rule_command) = splitted_line.pop_front() {
                                let remaining_line = Vec::from(splitted_line).join(" ");
                                let regex = Regex::new(&remaining_line).expect(&format!(
                                    "Could not compile the regex {}",
                                    &remaining_line
                                ));
                                match rule_command {
                                    "managed" => commands.push(Command::RuleManaged(regex)),
                                    "exclude" => commands.push(Command::RuleExclude(regex)),
                                    _ => panic!("Invalid rule subcommand"),
                                }
                            }
                        }
                        _ => panic!(
                            "Error while parding the configuration, command {} not found",
                            command
                        ),
                    }
                }
            }
        }

        commands
    }

    pub fn build(&self) -> Config {
        let mut managed_rule_regexes = Vec::new();
        let mut unmanaged_rule_regexes: Vec<Regex> = Vec::new();
        let mut workspaces: Vec<(usize, String)> = Vec::new();

        for command in self.commands.iter() {
            match command {
                Command::RuleExclude(regex) => unmanaged_rule_regexes.push(regex.clone()),
                Command::RuleManaged(regex) => managed_rule_regexes.push(regex.clone()),
                Command::Workspace(index, name) => workspaces.push((index.clone(), name.clone())),
            }
        }

        // Sort the workspaces by indexes
        workspaces.sort_by(|a, b| a.0.cmp(&b.0));

        Config {
            _excluded_windows: unmanaged_rule_regexes,
            managed_windows: managed_rule_regexes,
            workspaces_monitors: workspaces.into_iter().map(|item| item.1).collect(),
        }
    }
}
