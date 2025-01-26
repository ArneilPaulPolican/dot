use std::process::Command;
use std::path::Path;
use std::error::Error;
use tempfile::tempdir;
use mockall::predicate::*;
use mockall::*;

pub fn run_template(args: &[&str], template: &str) -> Result<(), Box<dyn Error>>{
    println!("Running {}...{:?}", template, args);

    let destination = format!("./templates/{}-template", template);
    let destination_path = Path::new(&destination);

    let valid_templates = ["minimal", "parachain", "solochain"];
    if !valid_templates.contains(&template) {
        return Err(format!("Template unrecognized: {}", template).into());
    }

    // Clone template
    let mut status = Command::new("true").status().expect("Failed to initialize status");
    if !destination_path.exists() {
        println!("\n↓ Let's grab the {} template from github.\n", template);
        status = Command::new("git")
            .args(&["clone", "--quiet", &format!("https://github.com/paritytech/polkadot-sdk-{}-template.git", template), &destination])
            .status()
            .expect("Failed to clone template");
    }

    println!("Entered directory: {}", destination);
    let repo_path = Path::new(&destination);
    if !status.success() {
        return Err(format!("Failed to clone template").into());
    }

    println!("args: {:?}", args);

    let _ = serve_template(args, repo_path);

    println!("{} is now running.", template);
    Ok(())
}

fn serve_template(args: &[&str], repo_path: &Path) -> Result<(), Box<dyn Error>>{
    if !repo_path.exists() {
        return Err(format!("The specified template directory does not exist: {:?}", repo_path).into());
    }

    let output = Command::new("cargo")
        .args(&["run", "--release", "--", "--dev"])
        .args(args)
        .current_dir(repo_path)
        .output()
        .expect("Failed to run project");

    println!("{}", String::from_utf8_lossy(&output.stdout));

    if !output.status.success() {
        return Err(format!("Failed to run a node").into());
    }
    Ok(()) 
}


/// =================================================================================================
/// Test Module
/// =================================================================================================
#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use std::fs;
    use std::io;
    use mockall::predicate::eq;

    // RUN TEMPLATE TESTS
    #[test]
    fn test_run_template_fail_template_not_recognize() {
        // Arrange
        let args = vec!["--arg1", "value1"];
        let template = "unknown_template"; // Use an unrecognized template

        // Act: Run the function you are testing
        let result = run_template(&args, template);

        // Assert: Check that the result is an error
        assert!(result.is_err(), "Expected run_template to return an error for unrecognized template");
        assert_eq!(result.unwrap_err().to_string(), format!("Template unrecognized: {}", template));
    
    }

    #[test]
    fn test_run_template_with_invalid_template() {
        let template = "invalid-template";
        let result = run_template(&[], template);
        assert!(result.is_err());
    }


    #[test]
    fn test_serve_template() -> Result<(), Box<dyn Error>> {
        // Create a temporary directory
        let temp_dir = tempdir()?;
        let repo_path = temp_dir.path();
    
        // Create a minimal Cargo project in the temporary directory
        fs::create_dir(repo_path.join("src"))?;
        fs::write(repo_path.join("Cargo.toml"), r#"
            [package]
            name = "test_project"
            version = "0.1.0"
            edition = "2018"
    
            [dependencies]
        "#)?;
        fs::write(repo_path.join("src/main.rs"), r#"
            fn main() {
                println!("Hello, world!");
            }
        "#)?;
    
        // Call the serve_template function
        serve_template(&["--example-arg"], repo_path)?;
    
        Ok(())
    }

    
    // SERVE TEMPLATE TESTS
    #[test]
    fn test_serve_template_fail_no_directory() {
        // Arrange
        // let args = vec!["--arg1", "value1", "--arg2", "value2"];
        let template = "mock";

        let destination = format!("./templates/{}-template", template);
        let destination_path = Path::new(&destination);

        assert!(!destination_path.exists(), "Template directory should exist");
    }

    #[test]
    fn test_serve_template_fail_no_file() {
        // Arrange
        let args = vec!["--arg1", "value1", "--arg2", "value2"];
        let template = "mock";

        let destination = format!("./templates/{}-template", template);
        let destination_path = Path::new(&destination);

        // Act: Run the function you are testing
        let result = serve_template(&args, destination_path);

        // Assert: Check if the result is an error
        assert!(result.is_err(), "Expected serve_template to return an error");
    }
}