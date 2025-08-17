use freedesktop_icons::lookup;
use raylib::ffi::{Image, LoadImageFromMemory, LoadTextureFromImage, UnloadImage};
use raylib::prelude::*;
use std::{env, ffi::CString};

use std::path::{Path, PathBuf};

static DEFAULT_ICON_DATA: &[u8] = include_bytes!("../resources/default.png");

pub fn find_binary(name: &str) -> PathBuf {
    let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".into());

    let mut exe_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    exe_path.push("target");
    exe_path.push(&profile);
    exe_path.push(format!("{}{}", name, env::consts::EXE_SUFFIX));

    exe_path
}

pub fn load_icon(
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
                "WARNING: Failed to convert icon path to str for {}",
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

pub fn load_default_icon() -> Result<Texture2D, String> {
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
        return Err("Failed to load default icon from bytes".to_string());
    }

    unsafe {
        let raw_texture = LoadTextureFromImage(image);
        UnloadImage(image);
        Ok(Texture2D::from_raw(raw_texture))
    }
}
