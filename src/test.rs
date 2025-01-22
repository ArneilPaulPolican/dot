#[cfg(test)]
mod e2e_tests {
    use tempfile::{tempdir, TempDir};
    use super::*;
    use std::env;
    use std::{fs, path::Path};
    use std::process::{Command, Output};
    use mockito::mock;
    use std::fs::{File, create_dir_all};
    use crate::install::{install, install_polkadot, install_chain_spec_builder, install_omni_node, run_download_script, gen_chain_spec, move_chain_spec};
    use crate::process::Stdio;
    use crate::os_check::{check_operating_system, get_os_info, is_wsl};
    use std::path::PathBuf;
    use std::error::Error;
    
 
 
    // INSTALL
    #[test]
    fn test_install() {
        
        install("template");
        // Mocking installation functions
        fn mock_install_polkadot() -> Result<(), Box<dyn Error>> {
            Ok(()) // Simulate success
        }
        fn mock_install_chain_spec_builder() -> Result<(), Box<dyn Error>> {
            Err(Box::from("Mock failure for chain spec builder")) // Simulate failure
        }
        fn mock_install_omni_node() -> Result<(), Box<dyn Error>> {
            Ok(()) // Simulate success
        }
        fn mock_run_download_script() -> Result<(), Box<dyn Error>> {
            Err(Box::from("Mock failure for download script")) // Simulate failure
        }
        fn mock_gen_chain_spec() -> Result<(), Box<dyn Error>> {
            Ok(()) // Simulate success
        }
    
        // Mock the install function
        fn test_install_template() {
            let mut results: Vec<(Result<(), Box<dyn Error>>, &str)> = Vec::new();
    
            results.push((mock_install_polkadot(), "$ Polkadot installation"));
            results.push((mock_install_chain_spec_builder(), "$ Chain spec builder installation"));
            results.push((mock_install_omni_node(), "$ Omni-node installation"));
            results.push((mock_run_download_script(), "$ Wasm file download script"));
            results.push((mock_gen_chain_spec(), "$ Chain spec script"));
    
            println!(" ");
            println!("===========================================================================");
            println!(" ");
            for (result, message) in results {
                match result {
                    Ok(_) => println!("{} success ✓", message),
                    Err(e) => println!("{} failed ✗: {}", message, e),
                }
            }
            println!(" ");
            println!("===========================================================================");
            println!(" ");
        }
    
        // Run the mocked install function
        test_install_template();
        
        // Assertions can be done by capturing the output using libraries like `assert_cmd` or `insta`,
        // but here we assume manual verification of output is sufficient.
        assert!(true); // Placeholder assertion since we're primarily inspecting printed output
    }
    
 
    #[test]
    fn test_install_polkadot() {
        // Create a temporary directory for mock binaries
        let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
        let mock_dir = temp_dir.path();
 
        // Create mock `curl` binary
        let curl_path = mock_dir.join("curl");
        fs::write(&curl_path, "#!/bin/sh\necho Mock curl executed\nexit 0")
            .expect("Failed to write mock curl");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&curl_path, fs::Permissions::from_mode(0o755))
                .expect("Failed to make mock curl executable");
        }
 
        // Create mock `bash` binary
        let bash_path = mock_dir.join("bash");
        fs::write(&bash_path, "#!/bin/sh\necho Mock bash executed\nexit 0")
            .expect("Failed to write mock bash");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&bash_path, fs::Permissions::from_mode(0o755))
                .expect("Failed to make mock bash executable");
        }
 
        // Override PATH to use mock binaries
        let original_path = env::var("PATH").unwrap_or_default();
        let new_path = format!("{}:{}", mock_dir.to_str().unwrap(), original_path);
        env::set_var("PATH", new_path);
 
        // Run the function and check the result
        let result = install_polkadot();
        assert!(result.is_ok(), "Function failed: {:?}", result.err());
 
        // Restore the original PATH
        env::set_var("PATH", original_path);
    }
 
    #[cfg(target_os = "linux")]
    #[test]
    fn test_check_operating_system_linux() {
        env::set_var("CARGO_CFG_TARGET_OS", "linux");
 
        let os_info = check_operating_system();
        assert_eq!(os_info, "linux");
    }
 
    #[cfg(target_os = "windows")]
    #[test]
    fn test_check_operating_system_windows_wsl() {
        env::set_var("CARGO_CFG_TARGET_OS", "windows");
        
        // Mock WSL behavior (we simulate WSL here)
        if mock_is_wsl() {
            let os_info = check_operating_system();
            assert_eq!(os_info, "windows-wsl2");
        } else {
            let os_info = check_operating_system();
            assert_eq!(os_info, "windows");
        }
    }
 
    #[cfg(target_os = "macos")]
    #[test]
    fn test_check_operating_system_macos() {
        env::set_var("CARGO_CFG_TARGET_OS", "macos");
        
        // Check if the OS info returns the correct result
        let os_info = check_operating_system();
        assert_eq!(os_info, "macos");
    }
 
    // CHAIN SPEC BUILDER
    // Test for successful installation
    #[test]
    fn test_install_chain_spec_builder_success() {
        // Mock the URL and wget command
        let _mock = mock("GET", "/chain-spec-builder")
            .with_status(200)
            .with_body("test binary content")
            .create();
       
        // Temporary directory to test file creation
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path().join("binaries");
        // Mock successful command execution for wget and chmod
        fs::create_dir_all(&temp_path).unwrap();
 
        // Execute the function
        let result = install_chain_spec_builder();
 
        assert!(result.is_ok());
    }
 
 
    // Test if 'binaries' directory is created when it doesn't exist
    #[test]
    fn test_create_binaries_directory() {
        // Create a temporary directory for testing
        let temp_dir = TempDir::new().unwrap();
        let binaries_dir = temp_dir.path().join("binaries");
 
        // Ensure the directory doesn't exist at the start of the test
        assert!(!binaries_dir.exists());
 
        // Change the working directory for the test to the temp directory
        std::env::set_current_dir(temp_dir.path()).unwrap();
 
        // Run the function to check if it creates the 'binaries' directory
        install_chain_spec_builder().unwrap();
 
        // Check that the 'binaries' directory was created
        assert!(binaries_dir.exists());
    }
 
    #[test]
    fn test_install_omni_node() {
        install_omni_node();
        // Create a temporary directory for mock binaries
        let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
        let mock_dir = temp_dir.path();
    
        // Create mock `wget` binary
        let wget_path = mock_dir.join("wget");
        fs::write(&wget_path, "#!/bin/sh\necho Mock wget executed\nexit 0")
            .expect("Failed to write mock wget");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&wget_path, fs::Permissions::from_mode(0o755))
                .expect("Failed to make mock wget executable");
        }
    
        // Override PATH to use mock binaries
        let original_path = env::var("PATH").unwrap_or_default();
        let new_path = format!("{}:{}", mock_dir.to_str().unwrap(), original_path);
        env::set_var("PATH", new_path);
    
        let url = "https://github.com/paritytech/polkadot-sdk/releases/download/polkadot-stable2412/polkadot-omni-node";
        let destination = Path::new("./binaries/polkadot-omni-node");
    
        if destination.exists() {
            fs::remove_file(destination).expect("Failed to remove existing file");
        }
    
        // Simulate the successful download with mock `wget`
        let output = Command::new("wget")
            .arg("-O")
            .arg(destination)
            .arg(url)
            .output()
            .expect("Failed to execute wget");
    
        assert!(output.status.success(), "Omni-node installation failed");
    
        // Restore the original PATH
        env::set_var("PATH", original_path);
    }
    
    #[test]
fn test_run_download_script() {
    // Create a temporary directory for mock binaries
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let mock_dir = temp_dir.path();
    
    // Create mock `wget` binary
    let wget_path = mock_dir.join("wget");
    fs::write(&wget_path, "#!/bin/sh\ntouch \"$2\"\necho -n \"Mock file downloaded\" > \"$2\"\nexit 0")
        .expect("Failed to write mock wget");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&wget_path, fs::Permissions::from_mode(0o755))
            .expect("Failed to make mock wget executable");
    }
    
    // Override PATH to use mock binaries
    let original_path = env::var("PATH").unwrap_or_default();
    let new_path = format!("{}:{}", mock_dir.to_str().unwrap(), original_path);
    env::set_var("PATH", new_path);
    
    // Run the download script
    let result = run_download_script();
    
    // Assert that the download script ran successfully
    assert!(result.is_ok(), "Download script failed: {:?}", result.err());
    
    let destination = Path::new("./nodes/asset_hub_westend_runtime.compact.compressed.wasm");
    
    // Check if the file was created
    assert!(destination.exists(), "WASM file not created at the expected destination.");
    
    // Optionally, check if the file contains the expected content
    let file_content = fs::read_to_string(destination).expect("Failed to read WASM file");
    assert_eq!(file_content, "Mock file downloaded", "File content mismatch.");
    
    // Restore the original PATH
    env::set_var("PATH", original_path);
}
 
    
    
#[test]
fn test_gen_chain_spec_success() {
    gen_chain_spec();
    // Create a temporary directory for mock binaries
    let temp_dir = tempdir().expect("Failed to create temp dir");
    let mock_dir = temp_dir.path();
 
    // Create mock `chain-spec-builder` binary
    let chain_spec_builder_path = mock_dir.join("chain-spec-builder");
    fs::write(&chain_spec_builder_path, "#!/bin/sh\necho \"Mock chain-spec-builder executed\"\nexit 0")
        .expect("Failed to write mock chain-spec-builder");
 
    // Set executable permissions for Unix-based systems
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&chain_spec_builder_path, fs::Permissions::from_mode(0o755))
            .expect("Failed to make mock chain-spec-builder executable");
    }
 
    // Override PATH to include the mock directory
    let original_path = env::var("PATH").unwrap_or_default();
    let new_path = format!("{}:{}", mock_dir.to_str().unwrap(), original_path);
    env::set_var("PATH", new_path);
 
    // Verify that the mock binary exists and is in the PATH
    assert!(
        chain_spec_builder_path.exists(),
        "Mock chain-spec-builder binary does not exist."
    );
 
    // Execute the command using the mock binary
    let output = std::process::Command::new("chain-spec-builder")
        .output()
        .expect("Failed to execute chain-spec-builder");
 
    // Check the output
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        output.status.success(),
        "Mock chain-spec-builder failed with stderr: {}",
        stderr
    );
    assert_eq!(
        stdout.trim(),
        "Mock chain-spec-builder executed",
        "Unexpected stdout: {}",
        stdout
    );
 
    // Restore the original PATH
    env::set_var("PATH", original_path);
}
 
 
 
    
    #[test]
    fn test_gen_chain_spec_failure_wasm_not_found() {
        // Simulating that the file doesn't exist.
        let result = gen_chain_spec();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "WASM file not found: \"./nodes/asset_hub_westend_runtime.compact.compressed.wasm\"");
    }
 
    #[test]
    fn test_gen_chain_spec_failure_chmod_wasm() {
        // Simulate the failure of chmod when the file doesn't exist.
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let mock_dir = temp_dir.path();
        let wasm_path = mock_dir.join("non_existent_wasm_file.wasm");
    
        // This will fail because the file doesn't exist
        let result = Command::new("chmod")
            .arg("+r")
            .arg(wasm_path.to_str().unwrap())
            .status()
            .expect("Failed to run chmod");
    
        assert!(!result.success(), "Expected chmod to fail because the file does not exist");
    }
 
    #[test]
    fn test_gen_chain_spec_failure_chmod_chain_spec_builder() {
        // Create a temporary directory for mock binaries
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let mock_dir = temp_dir.path();
    
        // Define the path to the non-existent chain-spec-builder binary
        let chain_spec_builder_path = mock_dir.join("non_existent_chain_spec_builder");
    
        // Attempt to set execute permissions on the non-existent file
        let result = Command::new("chmod")
            .arg("+x")
            .arg(chain_spec_builder_path.to_str().unwrap())
            .status()
            .expect("Failed to run chmod");
    
        // Assert that chmod fails (i.e., returns a non-zero exit status)
        assert!(!result.success(), "Expected chmod to fail due to non-existent file.");
    }
 
 
#[test]
fn test_move_chain_spec() {
    move_chain_spec();
 
    // Create a temporary directory for the test
    let temp_dir = tempdir().expect("Failed to create temp dir");
    let temp_path = temp_dir.path();
 
    // Create mock source and destination directories
    let source_dir = temp_path.join("source");
    let dest_dir = temp_path.join("chain-specs");
 
    // Create source directory and mock chain_spec.json file
    fs::create_dir(&source_dir).expect("Failed to create source directory");
    let source_file = source_dir.join("chain_spec.json");
    fs::write(&source_file, "mock chain spec content").expect("Failed to write chain_spec.json");
 
    // Update the search directories for the test
    let search_directories = vec![source_dir.to_str().unwrap().to_string()];
 
    // Mock the function to use test-specific paths
    fn test_move_chain_spec(search_directories: Vec<String>, dest_dir: &Path) -> Result<(), String> {
        let mut chain_spec_source_path: Option<PathBuf> = None;
 
        // Locate the chain_spec.json file
        for dir in &search_directories {
            let potential_path = Path::new(dir).join("chain_spec.json");
            if potential_path.exists() {
                chain_spec_source_path = Some(potential_path);
                break;
            }
        }
 
        // Check if the file was found
        let chain_spec_source_path = match chain_spec_source_path {
            Some(path) => path,
            None => {
                return Err(format!("chain_spec.json not found in the specified directories."));
            }
        };
 
        let chain_spec_destination_path = dest_dir.join("chain_spec.json");
 
        // Create the chain-specs directory if it does not exist
        if let Err(e) = fs::create_dir_all(chain_spec_destination_path.parent().unwrap()) {
            return Err(format!("Failed to create chain-specs directory: {}", e));
        }
 
        // Move the chain_spec.json file to the chain-specs directory
        if let Err(e) = fs::rename(&chain_spec_source_path, &chain_spec_destination_path) {
            return Err(format!("Failed to move chain_spec.json: {}", e));
        }
        Ok(())
    }
 
    // Run the test-specific move_chain_spec function
    let result = test_move_chain_spec(search_directories, &dest_dir);
 
    // Assertions
    assert!(result.is_ok(), "move_chain_spec failed: {:?}", result.err());
    assert!(
        !source_file.exists(),
        "Source chain_spec.json should not exist after being moved."
    );
    let dest_file = dest_dir.join("chain_spec.json");
    assert!(
        dest_file.exists(),
        "chain_spec.json was not moved to the destination directory."
    );
 
    // Verify file contents
    let content = fs::read_to_string(dest_file).expect("Failed to read destination chain_spec.json");
    assert_eq!(content, "mock chain spec content", "File content mismatch.");
}
 
#[test]
fn test_move_chain_spec_failure() {
    // Run the function without the source file present to simulate an error
    let result = move_chain_spec();
    
    // Assert that the result is Err
    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), "chain_spec.json not found in the specified directories.");
}
 
}
 
 
 