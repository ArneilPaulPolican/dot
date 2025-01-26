use std::process::{Command, ExitStatus};
use std::error::Error;
use std::io;

pub struct RealCommand {
    command: Command,
}

pub fn run(args: &[&str]) {
    println!("Running omni-node...");
    println!("args: {:?}", args);

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

pub fn run_eth<R: CommandRunner>(runner: &mut R, args: &[&str]) -> Result<(), Box<dyn Error>> {
    println!("Running eth-rpc...");
    println!("args: {:?}", args);

    // Pass the arguments to the runner
    runner.args(args);
    match runner.status() {
        Ok(status) if status.success() => {
            println!("Eth-rpc is now running.");
            Ok(())
        }
        Ok(status) => {
            eprintln!("Eth-roc failed to start with exit status: {}", status);
            Err(format!("Eth-roc failed to start with exit status: {}", status).into())
        }
        Err(e) => {
            eprintln!("Failed to run Eth-rpc: {}", e);
            Err(format!("Failed to run Eth-rpc: {}", e).into())
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


/// =================================================================================================
/// Test Module
/// =================================================================================================
#[cfg(test)]
mod tests {
    use super::*;
    use std::process::ExitStatus;
    use std::os::unix::process::ExitStatusExt;

    // Mock Command for testing purposes
    struct MockCommand {
        args: Vec<String>,
        success: bool,
        fail_to_run: bool,
    }

    impl CommandRunner for MockCommand {
        fn new(_program: &str) -> Self {
            MockCommand {
                args: Vec::new(),
                success: true,
                fail_to_run: false,
            }
        }

        fn args(&mut self, args: &[&str]) -> &mut Self {
            self.args.extend(args.iter().map(|&arg| arg.to_string()));
            self
        }

        fn status(&mut self) -> io::Result<ExitStatus> {
            if self.fail_to_run {
                Err(io::Error::new(io::ErrorKind::Other, "Simulated failure"))
            } else if self.success {
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
    #[test]
    fn test_run_fail_to_run() {
        let mut mock_command = MockCommand::new("./mock-path");
        mock_command.fail_to_run = true;

        // Call run with mock behavior
        run(&["--chain", "./mock-specs/mock_chain.json"]);

        // Add assertions as needed
    }

    #[test]
    fn test_run_eth_success() {
        let mut mock_command = MockCommand::new("./mock-path");
        mock_command.success = true;

        // Call run_eth with mock behavior
        let result = run_eth(&mut mock_command, &["--chain", "./mock-specs/mock_chain.json"]);
        assert!(result.is_ok());

        // Add assertions as needed
    }

    #[test]
    fn test_run_eth_failure() {
        let mut mock_command = MockCommand::new("./mock-path");
        mock_command.success = false;

        // Call run_eth with mock behavior
        let result = run_eth(&mut mock_command, &["--chain", "./mock-specs/mock_chain.json"]);
        assert!(result.is_err());

        // Add assertions as needed
    }

    #[test]
    fn test_run_eth_runner_failure() {
        struct FailingCommand;

        impl CommandRunner for FailingCommand {
            fn new(_program: &str) -> Self {
                FailingCommand
            }

            fn args(&mut self, _args: &[&str]) -> &mut Self {
                self
            }

            fn status(&mut self) -> io::Result<ExitStatus> {
                Err(io::Error::new(io::ErrorKind::Other, "Simulated failure"))
            }
        }

        let mut failing_command = FailingCommand::new("./mock-path");

        // Call run_eth with failing runner
        let result = run_eth(&mut failing_command, &["--chain", "./mock-specs/mock_chain.json"]);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Failed to run Eth-rpc: Simulated failure");

        // Add assertions as needed
    }
}
