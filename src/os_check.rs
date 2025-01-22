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

pub fn is_wsl() -> bool {
    std::path::Path::new("/proc/version").exists() && 
    std::fs::read_to_string("/proc/version").unwrap_or_default().contains("Microsoft")
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    // Mocking the behavior of `is_wsl` for different environments
    fn mock_is_wsl() -> bool {
        if cfg!(target_os = "linux") {
            true
        } else if cfg!(target_os = "windows") {
            false
        } else {
            false
        }
    }

    #[test]
    fn test_get_os_info(){
        let os = env::consts::OS;
        match os {
            "macos" => {
                assert_eq!(get_os_info(), "macos");
            },
            "linux" => {
                assert_eq!(get_os_info(), "linux");
            },
            "windows" => {
                if mock_is_wsl() {
                    assert_eq!(get_os_info(), "windows-wsl2");
                } else {
                    assert_eq!(get_os_info(), "windows");
                }
            },
            _ => {
                assert_eq!(get_os_info(), format!("Unknown operating system: {}", os));
            }
        }
    }

    #[test]
    fn test_check_operating_system(){
        let os = env::consts::OS;
        match os {
            "macos" => {
                assert_eq!(check_operating_system(), "macos");
            },
            "linux" => {
                assert_eq!(check_operating_system(), "linux");
            },
            "windows" => {
                if mock_is_wsl() {
                    assert_eq!(check_operating_system(), "windows-wsl2");
                } else {
                    assert_eq!(check_operating_system(), "windows");
                }
            },
            _ => {
                assert_eq!(check_operating_system(), format!("Unknown operating system: {}", os));
            }
        }
    }

    #[test]
    fn test_is_wsl() {
        if cfg!(target_os = "linux") {
            let result = is_wsl();
            if std::path::Path::new("/proc/version").exists() {
                let content = std::fs::read_to_string("/proc/version").unwrap_or_default();
                if content.contains("Microsoft") {
                    assert!(
                        result,
                        "Expected `is_wsl` to return true in a WSL environment on Linux."
                    );
                } else {
                    assert!(
                        !result,
                        "Expected `is_wsl` to return false on native Linux."
                    );
                }
            } else {
                assert!(
                    !result,
                    "`is_wsl` should return false when `/proc/version` does not exist."
                );
            }
        } else if cfg!(target_os = "windows") {
            assert!(
                !is_wsl(),
                "`is_wsl` should return false on native Windows."
            );
        } else if cfg!(target_os = "macos") {
            assert!(
                !is_wsl(),
                "`is_wsl` should return false on macOS."
            );
        } else {
            panic!("Unsupported environment for testing.");
        }
    }
}

