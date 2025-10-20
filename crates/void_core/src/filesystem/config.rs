use dirs_next::home_dir;
use std::path::PathBuf;

/// # Setups config dir at first run
/// requires vec of subdirectories that will be created under .config/void/folder
/// ```
/// use void_core::filesystem::config::init_config_folder;
/// let config = init_config_folder(vec!["db", "plugns", "themes", "fonts"]);
/// ```
/// if you want to create config dir without nested directories you can do it like this:
/// ```
/// use void_core::filesystem::config::init_config_folder;
/// let config = init_config_folder(vec![]);
/// ```
pub fn init_config_folder(config_subdirs: Vec<&'static str>) {
    let home_dir = match home_dir() {
        Some(s) => s.join(".config").join("void"),
        None => {
            error!("Home dir not found (this should be bug. Write an issue in our repo)");
            panic!("Home dir not found (this should be bug. Write an issue in our repo)")
        }
    };
    std::fs::create_dir_all(&home_dir).unwrap_or_else(|_| panic!("read-only filesystem"));
    for entry in config_subdirs {
        let temp_subdir = home_dir.join(entry);
        let _ = std::fs::create_dir(temp_subdir).map_err(|_| {
            error!("Failed to create config subdirectory");
        });
    }
    info!("Created config folder");
}

/// # returns PathBuf to current config folder or creates it if not exist.
/// it could return error if config folder is not initialized
/// requires Option subpath to get subdirectories under config folder
/// ```
/// use void_core::filesystem::config::get_config_folder;
/// //void_core.init_config_dir();
/// get_config_folder(None); //returns PathBuf
/// ```
/// if init_config_dir not called will call it inside
pub fn get_config_folder(subpath: Option<&'static str>) -> PathBuf {
    let dir = match subpath {
        Some(path) => home_dir().unwrap().join(".config").join("void").join(path),
        None => home_dir().unwrap().join(".config").join("void"),
    };
    match std::fs::read_dir(&dir) {
        Ok(_) => dir,
        Err(_) => match subpath {
            Some(s) => {
                init_config_folder(vec![s]);
                dir
            }
            None => {
                init_config_folder(vec![]);
                dir
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_creation_with_error() {
        let test_path = home_dir().unwrap().join(".config").join("void");
        init_config_folder(vec!["test", "test", "test"]);
        std::fs::remove_dir_all(&test_path).unwrap();
    }
}
