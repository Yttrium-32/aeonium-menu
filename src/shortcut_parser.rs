use std::ffi::CString;
use std::path::{Path, PathBuf};

use freedesktop_entry_parser::parse_entry;
use freedesktop_icons::lookup;

use raylib::ffi::{Image, LoadImageFromMemory, LoadTextureFromImage, UnloadImage};
use raylib::{RaylibHandle, RaylibThread, texture::Texture2D};

static DEFAULT_ICON_DATA: &[u8] = include_bytes!("../resources/default.png");

#[derive(Debug)]
pub struct DesktopFile {
    name: String,
    exec_path: PathBuf,
    exec_args: Vec<String>,
    pub icon: Texture2D,
}

pub fn get_shortcut_files(
    config_dir: &Path,
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
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
        match parse_shortcut_file(&path, rl, thread) {
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

fn parse_shortcut_file(
    file_path: impl AsRef<Path>,
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
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

    let icon = load_icon(rl, thread, desktop_section.attr("Icon"), file_path.as_ref())?;

    Ok(DesktopFile {
        name,
        exec_path: total_exec_cmd[0].into(),
        exec_args: total_exec_cmd[1..].iter().map(|&s| s.to_string()).collect(),
        icon,
    })
}

fn load_icon(
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
    icon_field: Option<&str>,
    file_path: &Path,
) -> Result<Texture2D, String> {
    let field = match icon_field {
        Some(field) => field,
        None => {
            eprintln!("WARNING: No `Icon` field in {}", file_path.display());
            eprintln!("WARNING: Loading default icon");
            return load_default_icon();
        }
    };

    let icon_path = match lookup(field).find() {
        Some(icon_path) => icon_path,
        None => {
            eprintln!(
                "WARNING: Failed to find icon path for {}",
                file_path.display()
            );
            eprintln!("WARNING: Loading default icon");
            return load_default_icon();
        }
    };

    let path_str = match icon_path.to_str() {
        Some(path_str) => path_str,
        None => {
            eprintln!(
                "WARNING: Failed to icon path to str for {}",
                file_path.display()
            );
            eprintln!("WARNING: Loading default icon");
            return load_default_icon();
        }
    };

    match rl.load_texture(thread, path_str) {
        Ok(texture) => Ok(texture),
        Err(_) => {
            eprintln!(
                "WARNING: Failed to load icon texture for {}",
                file_path.display()
            );
            eprintln!("WARNING: Loading default icon");
            load_default_icon()
        }
    }
}

#[inline(always)]
fn load_default_icon() -> Result<Texture2D, String> {
    let extension = CString::new(".png")
        .map_err(|_| "Failed to convert file extension to CString".to_string())?;

    let image: Image = unsafe {
        LoadImageFromMemory(
            extension.as_ptr(),
            DEFAULT_ICON_DATA.as_ptr(),
            DEFAULT_ICON_DATA.len() as i32,
        )
    };

    if image.data.is_null() {
        return Err("Failed to default icon from bytes".to_string());
    }

    unsafe {
        let raw_texture = LoadTextureFromImage(image);
        UnloadImage(image);
        Ok(Texture2D::from_raw(raw_texture))
    }
}
