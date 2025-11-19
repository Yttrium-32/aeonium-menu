use std::fs;
use std::path::Path;

use anyhow::Context;
use resvg::tiny_skia::Pixmap;
use resvg::usvg::{self, Transform};

#[inline]
pub fn is_svg(data: impl AsRef<[u8]>) -> bool {
    let inner = |data: &[u8]| -> bool {
        let opt = usvg::Options::default();
        usvg::Tree::from_data(data, &opt).is_ok()
    };
    inner(data.as_ref())
}
fn load_svg(file_path: impl AsRef<Path>) -> Result<usvg::Tree, usvg::Error> {
    let svg_contents = fs::read_to_string(file_path)
        .expect("Couldn't read from file");
    let tree = usvg::Tree::from_str(&svg_contents, &usvg::Options::default())?;
    Ok(tree)
}

fn render_svg(tree: &usvg::Tree) -> Pixmap {
    let size = tree.size();
    let mut pixmap = Pixmap::new(size.width() as u32, size.height() as u32)
        .expect("Couldn't create new pixmap");
    resvg::render(tree, Transform::default(), &mut pixmap.as_mut());
    pixmap
}

pub fn convert_to_svg<P>(file_path: P, destination: P) -> anyhow::Result<()>
where
    P: AsRef<Path> + std::fmt::Debug + std::marker::Copy,
{
    let tree = load_svg(file_path)?;
    let pixmap = render_svg(&tree);
    pixmap.save_png(destination)
        .with_context(|| format!("Failed to write cached icon at {:?}", destination))?;
    Ok(())
}

