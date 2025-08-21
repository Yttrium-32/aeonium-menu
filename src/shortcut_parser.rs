use anyhow::{Context, bail};
use freedesktop_entry_parser::parse_entry;
use std::path::{Path, PathBuf};
use tracing::warn;

#[derive(Debug)]
pub struct DesktopFile {
    name: String,
    exec_path: PathBuf,
    exec_args: Vec<String>,
    pub icon: Option<PathBuf>,
}

pub fn get_shortcuts(config_dir: &Path) -> anyhow::Result<Vec<DesktopFile>> {
    let mut desktop_files = Vec::new();

    let shortcuts_dir = config_dir.join("shortcuts");
    if !shortcuts_dir.exists() {
        bail!(
            "Shortcuts directory does not exist: {}",
            shortcuts_dir.display()
        );
    }

    let entries = std::fs::read_dir(&shortcuts_dir).with_context(|| {
        format!(
            "Failed to read shortcuts directory: {}",
            shortcuts_dir.display()
        )
    })?;

    let desktop_paths: Vec<PathBuf> = entries
        .flatten()
        .filter(|entry| entry.path().extension().and_then(|ext| ext.to_str()) == Some("desktop"))
        .map(|entry| entry.path())
        .collect();

    if desktop_paths.is_empty() {
        bail!(
            "No desktop files found in directory: {}",
            shortcuts_dir.display()
        );
    }

    for path in desktop_paths {
        match parse_file(&path) {
            Ok(desktop_file) => desktop_files.push(desktop_file),
            Err(e) => {
                warn!(error = ?e, file = %path.display(), "Failed to parse desktop file");
            }
        }
    }

    Ok(desktop_files)
}

fn parse_file(file_path: impl AsRef<Path>) -> anyhow::Result<DesktopFile> {
    let file_path = file_path.as_ref();
    let entry = parse_entry(file_path)
        .with_context(|| format!("Failed to parse {}", file_path.display()))?;

    let desktop_section = entry.section("Desktop Entry");

    let name = desktop_section
        .attr("Name")
        .with_context(|| format!("No `Name` section found in {}", file_path.display()))?;

    let exec_attr = desktop_section
        .attr("Exec")
        .with_context(|| format!("No `Exec` section found in {}", file_path.display()))?;

    let total_exec_cmd: Vec<&str> = exec_attr.split_whitespace().collect();
    let (exec_path, exec_args) = total_exec_cmd
        .split_first()
        .with_context(|| format!("`Exec` field in {} is empty", file_path.display()))?;

    let icon = match desktop_section.attr("Icon") {
        Some(field) => Some(PathBuf::from(field)),
        None => {
            tracing::warn!(
                "No `Icon` field in {}, falling back to default",
                file_path.display()
            );
            None
        }
    };

    Ok(DesktopFile {
        name: name.to_string(),
        exec_path: PathBuf::from(exec_path),
        exec_args: exec_args.iter().map(|&s| s.to_string()).collect(),
        icon,
    })
}
