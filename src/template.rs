use std::process::{ExitStatus, Command};
use std::path::{Path, PathBuf};
use std::io::{self, Write};
use std::fs;
use std::sync::{Arc, Mutex};

pub struct RealCommand {
    command: Command,
}

pub fn run_template(args: &[&str], template: &str) {
    println!("Running {}...{:?}", template, args);

    let destination = format!("./templates/{}-template", template);
    let destination_path = Path::new(&destination);

    if destination_path.exists() {
        println!("\n✅︎ {}-template directory already exists at {}. -> Entering.\n", template, destination);
    } else {
        println!("\n↓ Let's grab the {} template from github.\n", template);
        let status = RealCommand::new("git")
            .args(&["clone", "--quiet", &format!("https://github.com/paritytech/polkadot-sdk-{}-template.git", template), &destination])
            .status()
            .expect("Failed to clone template");

        if !status.success() {
            eprintln!("Failed to clone template");
            return;
        }
    }

    println!("Entered directory: {}", destination);

    let repo_path = Path::new(&destination);
    println!("args: {:?}", args);

    let output = RealCommand::new("cargo")
        .args(&["run", "--release", "--", "--dev"])
        .args(args)
        .current_dir(repo_path)
        .command.output()
        .expect("Failed to run project");

    println!("{}", String::from_utf8_lossy(&output.stdout));

    if !output.status.success() {
        eprintln!("Failed to run project");
        return;
    }

    // Print the output to check during tests
    println!("{} is now running.", template);
}

pub trait CommandRunner {
    fn new(program: &str) -> Self;
    fn args(&mut self, args: &[&str]) -> &mut Self;
    fn current_dir(&mut self, dir: &Path) -> &mut Self;
    fn status(&mut self) -> std::io::Result<std::process::ExitStatus>;
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

    fn current_dir(&mut self, dir: &Path) -> &mut Self {
        self.command.current_dir(dir);
        self
    }

    fn status(&mut self) -> std::io::Result<std::process::ExitStatus> {
        self.command.status()
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::process::ExitStatus;
    use std::os::unix::process::ExitStatusExt;

    #[test]
    fn test_run_template_real() {
        let args = vec!["--chain-spec", "./chain-specs/mock_spec.json"];
        let template = "solochain";
        
        // This will call the real `run_template` function
        run_template(args.as_slice(), template);
        
        // In real cases, you'd need to check external side effects,
        // like if the right template was cloned or if the command ran successfully.
    }

    // Mock implementation of the CommandRunner trait
    struct MockCommand {
        program: String,
        args: Vec<String>,
        current_dir: Option<PathBuf>,
        exit_status: ExitStatus,
    }

    impl CommandRunner for MockCommand {
        fn new(program: &str) -> Self {
            MockCommand {
                program: program.to_string(),
                args: vec![],
                current_dir: None,
                exit_status: ExitStatus::from_raw(0), // Simulate success by default
            }
        }

        fn args(&mut self, args: &[&str]) -> &mut Self {
            self.args.extend(args.iter().map(|&s| s.to_string()));
            self
        }

        fn current_dir(&mut self, dir: &Path) -> &mut Self {
            self.current_dir = Some(dir.to_path_buf());
            self
        }

        fn status(&mut self) -> io::Result<ExitStatus> {
            Ok(self.exit_status)
        }
    }

    // #[test]
    // fn test_run_template_real() {
    //     let args = vec!["--chain-spec", "./chain-specs/mock_spec.json"];
    //     let template = "solochain";

    //     let mock_command = Arc::new(Mutex::new(MockCommand::new("mock_program")));

    //     // Call the function with the mock command
    //     run_template_with_mock(args.as_slice(), template, &mut *mock_command.lock().unwrap());

    //     let mock = mock_command.lock().unwrap();
    //     println!("Mock arguments: {:?}", mock.args);

    //     // Ensure the arguments passed to the mock command are as expected
    //     assert!(mock.args.contains(&"--chain-spec".to_string()), "Expected '--chain-spec' in args");
    //     assert!(mock.args.contains(&"./chain-specs/mock_spec.json".to_string()), "Expected './chain-specs/mock_spec.json' in args");
    // }

    // fn run_template_with_mock(args: &[&str], template: &str, command: &mut impl CommandRunner) {
    //     let destination = format!("./templates/{}-template", template);
    //     let destination_path = Path::new(&destination);

    //     // Simulate cloning template (no actual git operation)
    //     if destination_path.exists() {
    //         println!("\n✅︎ {}-template directory already exists at {}. -> Entering.\n", template, destination);
    //     } else {
    //         println!("\n↓ Let's grab the {} template from github.\n", template);
    //         command
    //             .args(&["clone", "--quiet", &format!("https://github.com/paritytech/polkadot-sdk-{}-template.git", template), &destination])
    //             .status()
    //             .unwrap();  // Simulate successful cloning
    //     }

    //     // Simulate running the cargo command (no actual cargo run)
    //     let status = command
    //         .args(&["run", "--release", "--", "--dev"])
    //         .args(args)
    //         .current_dir(Path::new(&destination))
    //         .status()
    //         .unwrap();

    //     // Ensure the command was "successful"
    //     assert!(status.success(), "Cargo run command failed");
    // }

}
