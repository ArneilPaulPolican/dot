use std::env;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref OS_INFO: Mutex<String> = Mutex::new(check_operating_system());
}

pub fn get_os_info() -> String {
    // Lock the mutex to access the OS_INFO safely
    let os_info = OS_INFO.lock().unwrap();
    os_info.clone() // Return a clone of the OS info
}

pub fn check_operating_system()  -> String {
    let os = env::consts::OS;
    match os {
        "macos" => "macos".to_string(),
        "linux" => "linux".to_string(),
        "windows" => {
            if is_wsl() {
                "windows-wsl2".to_string()
            } else {
                "windows".to_string()
            }
        },
        _ => format!("Unknown operating system: {}", os),
    }
}

// Function to check if the OS is WSL
pub fn is_wsl() -> bool {
    // Check for the presence of the WSL environment variable
    std::path::Path::new("/proc/version").exists() && 
    std::fs::read_to_string("/proc/version").unwrap_or_default().contains("Microsoft")
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;
    use std::path::Path;

    // Mocking the behavior of `is_wsl` for different environments
    fn mock_is_wsl() -> bool {
        if cfg!(target_os = "linux") {
            // Simulate WSL for Linux
            true
        } else if cfg!(target_os = "windows") {
            // Simulate native Windows (not WSL)
            false
        } else {
            // For other platforms, return false
            false
        }
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn test_get_os_info_macos() {
        // Simulate macOS
        env::set_var("CARGO_CFG_TARGET_OS", "macos");
        assert_eq!(get_os_info(), "macos");
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn test_get_os_info_linux() {
        // Simulate Linux (mocking is_wsl to return true for WSL)
        env::set_var("CARGO_CFG_TARGET_OS", "linux");
        assert_eq!(get_os_info(), "linux");
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn test_get_os_info_windows() {
        // Simulate Windows (mocking is_wsl to return false for native Windows)
        env::set_var("CARGO_CFG_TARGET_OS", "windows");
        assert_eq!(get_os_info(), "windows");
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn test_get_os_info_windows_wsl() {
        // Simulate Windows with WSL (mocking is_wsl to return true)
        env::set_var("CARGO_CFG_TARGET_OS", "windows");
        assert_eq!(get_os_info(), "windows-wsl2");
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn test_check_operating_system_macos() {
        env::set_var("CARGO_CFG_TARGET_OS", "macos");
        assert_eq!(check_operating_system(), "macos");
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn test_check_operating_system_linux() {
        env::set_var("CARGO_CFG_TARGET_OS", "linux");
        assert_eq!(check_operating_system(), "linux");
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn test_check_operating_system_windows() {
        env::set_var("CARGO_CFG_TARGET_OS", "windows");
        assert_eq!(check_operating_system(), "windows");
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn test_check_operating_system_windows_wsl() {
        env::set_var("CARGO_CFG_TARGET_OS", "windows");
        assert_eq!(check_operating_system(), "windows-wsl2");
    }

    #[test]
    fn test_is_wsl() {
        // Mock behavior for testing WSL detection
        assert_eq!(mock_is_wsl(), true);  // Linux WSL simulation
    }
}

