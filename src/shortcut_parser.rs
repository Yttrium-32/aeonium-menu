use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::{env, fs};

use anyhow::{Context, bail};
use directories::ProjectDirs;
use freedesktop_entry_parser::parse_entry;
use freedesktop_icons::lookup;
use tracing::{info, warn};

use crate::svg_utils::{convert_to_svg, is_svg};

#[derive(Debug)]
pub struct DesktopFile {
    name: String,
    exec_path: PathBuf,
    exec_args: Vec<String>,
    pub icon: Option<PathBuf>,
}

pub fn get_shortcuts(proj_dirs: &ProjectDirs) -> anyhow::Result<Vec<DesktopFile>> {
    let config_dir = proj_dirs.config_dir();
    info!("Found config directory: {}", config_dir.display());

    let mut desktop_files = Vec::new();

    let shortcuts_dir = if config_dir.join("shortcuts").exists() {
        config_dir.join("shortcuts")
    } else {
        PathBuf::from(
            env::var("HOME")
                .context("HOME not set, cannot fallback to ~/.local/share/applications")?,
        )
        .join(".local/share/applications")
    };

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
        match DesktopFile::new(&path, proj_dirs) {
            Ok(desktop_file) => desktop_files.push(desktop_file),
            Err(e) => {
                warn!("Error parsing {}: {e}", path.display());
            }
        }
    }

    Ok(desktop_files)
}

impl DesktopFile {
    fn new(file_path: impl AsRef<Path>, proj_dirs: &ProjectDirs) -> anyhow::Result<Self> {
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

        let mut icon = desktop_section.attr("Icon").and_then(|field| {
            lookup(field)
                .with_size(512)
                .with_cache()
                .find()
                .or_else(|| {
                    warn!("Icon doesn't exist: {}", field);
                    None
                })
        });

        if icon.is_none() {
            warn!(
                "No `Icon` field in {}, falling back to default",
                file_path.display()
            );
        } else {
            let icon_path = icon.as_ref().with_context(|| "Icon path was None")?;
            let stem = Path::new(icon_path)
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy();

            let icon_data = fs::read(icon_path)
                .with_context(|| format!("Failed to read from `{:?}`", icon_path))?;

            if is_svg(icon_data) {
                let cache_dir = proj_dirs.cache_dir();
                if !cache_dir.exists() {
                    fs::create_dir_all(cache_dir)?;
                }
                let png_path = cache_dir.join(format!("{}.png", stem));
                if !png_path.exists() {
                    convert_to_svg(icon_path, &png_path)?;
                }
                icon = Some(png_path);
            }
        }

        Ok(Self {
            name: name.to_string(),
            exec_path: PathBuf::from(exec_path),
            exec_args: exec_args.iter().map(|&s| s.to_string()).collect(),
            icon,
        })
    }

    pub fn spawn_process(&self) -> anyhow::Result<()> {
        let mut child_proc = Command::new(&self.exec_path);

        child_proc
            .args(&self.exec_args)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null());

        unsafe {
            child_proc.pre_exec(|| {
                if libc::setsid() == -1 {
                    return Err(std::io::Error::last_os_error());
                }
                Ok(())
            });
        }

        child_proc
            .spawn()
            .with_context(|| format!("Failed to spawn child process for {}", self.name))?;

        info!("Succesfully spawned {}", self.name);

        Ok(())
    }
}
