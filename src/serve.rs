use std::process::{Command, ExitStatus};
use std::io;

pub struct RealCommand {
    command: Command,
}

pub fn run(args: &[&str]) {
    println!("Running omni-node...");

    let command = RealCommand::new("./binaries/polkadot-omni-node")
        .args(args) 
        .status();

    match command {
        Ok(status) if status.success() => {
            println!("Omni-node is now running.");
        }
        Ok(status) => {
            eprintln!("Omni-node failed to start with exit status: {}", status);
        }
        Err(e) => {
            eprintln!("Failed to run omni-node: {}", e);
        }
    }
}

pub trait CommandRunner {
    fn new(program: &str) -> Self;
    fn args(&mut self, args: &[&str]) -> &mut Self;
    fn status(&mut self) -> io::Result<ExitStatus>;
}

impl CommandRunner for RealCommand {
    fn new(program: &str) -> Self {
        RealCommand {
            command: Command::new(program),
        }
    }

    fn args(&mut self, args: &[&str]) -> &mut Self {
        self.command.args(args);
        self
    }

    fn status(&mut self) -> io::Result<ExitStatus> {
        self.command.status()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::process::ExitStatus;
    use std::os::unix::process::ExitStatusExt;

    // Mock Command for testing purposes
    struct MockCommand {
        args: Vec<String>,
        success: bool,
    }

    impl CommandRunner for MockCommand {
        fn new(_program: &str) -> Self {
            MockCommand {
                args: Vec::new(),
                success: true,
            }
        }

        fn args(&mut self, args: &[&str]) -> &mut Self {
            self.args.extend(args.iter().map(|&arg| arg.to_string()));
            self
        }

        fn status(&mut self) -> io::Result<ExitStatus> {
            if self.success {
                Ok(ExitStatus::from_raw(0)) // Mock success
            } else {
                Ok(ExitStatus::from_raw(1)) // Mock failure
            }
        }
    }

    #[test]
    fn test_run_success() {
        let mut mock_command = MockCommand::new("./mock-path");
        mock_command.success = true;

        // Call run with mock behavior
        run(&["--chain", "./mock-specs/mock_chain.json"]);

        // Add assertions as needed
    }

    #[test]
    fn test_run_failure() {
        let mut mock_command = MockCommand::new("./mock-path");
        mock_command.success = false;

        // Call run with mock behavior
        run(&["--chain", "./mock-specs/mock_chain.json"]);

        // Add assertions as needed
    }

}
