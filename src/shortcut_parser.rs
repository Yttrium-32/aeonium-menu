use freedesktop_entry_parser::parse_entry;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct DesktopFile {
    name: String,
    exec_path: PathBuf,
    exec_args: Vec<String>,
    pub icon: Option<PathBuf>,
}

pub fn get_shortcuts(
    config_dir: &Path,
) -> Vec<DesktopFile> {
    let mut desktop_files = Vec::new();

    let shortcuts_dir = config_dir.join("shortcuts");
    if !shortcuts_dir.exists() {
        panic!(
            "ERROR: Shortcuts directory does not exist: {}",
            shortcuts_dir.display()
        );
    }

    let entries =
        std::fs::read_dir(&shortcuts_dir).expect("ERROR: Failed to read shortcuts directory");

    let desktop_paths: Vec<_> = entries
        .flatten()
        .filter(|entry| entry.path().extension().and_then(|ext| ext.to_str()) == Some("desktop"))
        .map(|entry| entry.path())
        .collect();

    if desktop_paths.is_empty() {
        panic!(
            "ERROR: No desktop files found in directory: {}",
            shortcuts_dir.display()
        );
    }

    for path in desktop_paths {
        match parse_file(&path) {
            Ok(desktop_file) => desktop_files.push(desktop_file),
            Err(e) => eprintln!(
                "WARNING: Failed to parse desktop file {}: {}",
                path.display(),
                e
            ),
        }
    }

    desktop_files
}

fn parse_file(
    file_path: impl AsRef<Path>,
) -> Result<DesktopFile, String> {
    let entry = match parse_entry(file_path.as_ref()) {
        Ok(val) => val,
        Err(err) => return Err(err.to_string()),
    };

    let desktop_section = entry.section("Desktop Entry");

    let name = match desktop_section.attr("Name") {
        Some(val) => val.to_string(),
        None => return Err("No `Name` section found in file".to_string()),
    };

    let total_exec_cmd = match desktop_section.attr("Exec") {
        Some(val) => val.split_whitespace().collect::<Vec<&str>>(),
        None => return Err("No `Exec` section found in file".to_string()),
    };

    let icon = match desktop_section.attr("Icon") {
        Some(field) => Some(field.into()),
        None => {
            eprintln!("WARNING: No `Icon` field in {}", file_path.as_ref().display());
            eprintln!("WARNING: Loading default icon");
            None
        }
    };

    Ok(DesktopFile {
        name,
        exec_path: total_exec_cmd[0].into(),
        exec_args: total_exec_cmd[1..].iter().map(|&s| s.to_string()).collect(),
        icon,
    })
}
