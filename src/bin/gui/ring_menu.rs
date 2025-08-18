use raylib::core::color::Color;
use raylib::core::drawing::RaylibDraw;
use raylib::drawing::RaylibDrawHandle;
use raylib::math::{Rectangle, Vector2};
use raylib::prelude::RaylibTexture2D;
use raylib::texture::Texture2D;

const COLOR_TRANSLUCENT_BLUE: Color = Color::new(100, 149, 237, 77);
const COLOR_DARK_BLUE: Color = Color::new(31, 102, 229, 220);

pub fn draw(
    d: &mut RaylibDrawHandle,
    screen_h: f32,
    screen_w: f32,
    highlight: Option<usize>,
    segments: usize,
    icon_textures: &[Texture2D]
) {
    assert_eq!(icon_textures.len(), segments, "ERR: icon_textures length mismatch");

    let center = Vector2::new(screen_w / 2.0, screen_h / 2.0);
    let outer_radius = screen_h.min(screen_w) * 0.25;
    let inner_radius = outer_radius * 0.75;

    let gap_angle = 2.0;
    let total_gap = gap_angle * segments as f32;
    let angle_per_segment = (360.0 - total_gap) / segments as f32;

    let mut start_angle = -90.0;

    for (idx, icon_tex) in icon_textures.iter().enumerate() {
        let end_angle = start_angle + angle_per_segment;

        let color = match highlight {
            Some(h_idx) => {
                assert!(
                    h_idx < segments,
                    "hightlight index {} out of bounds for segments {}",
                    h_idx,
                    segments
                );
                if h_idx == idx {
                    COLOR_DARK_BLUE
                } else {
                    COLOR_TRANSLUCENT_BLUE
                }
            }
            None => COLOR_TRANSLUCENT_BLUE,
        };

        d.draw_ring(
            center,
            inner_radius,
            outer_radius,
            start_angle,
            end_angle,
            0,
            color,
        );

        draw_icon(
            d,
            center,
            inner_radius,
            outer_radius,
            start_angle,
            end_angle,
            icon_tex
        );

        start_angle = end_angle + gap_angle;
    }
}

fn draw_icon(
    d: &mut RaylibDrawHandle,
    center: Vector2,
    inner_radius: f32,
    outer_radius: f32,
    start_angle: f32,
    end_angle: f32,
    icon: &Texture2D,
) {
    let mid_angle = (start_angle + end_angle) / 2.0;
    let mid_radius = (inner_radius + outer_radius) / 2.0;

    let mid_angle_rad = mid_angle.to_radians();

    let icon_pos = Vector2::new(
        center.x + mid_radius * mid_angle_rad.cos(),
        center.y + mid_radius * mid_angle_rad.sin(),
    );

    let max_icon_size = (outer_radius - inner_radius) * 0.7;

    let icon_width = icon.width() as f32;
    let icon_height = icon.height() as f32;
    let scale_factor = if icon_width > icon_height {
        max_icon_size / icon_width
    } else {
        max_icon_size / icon_height
    };

    let scaled_width = icon_width * scale_factor;
    let scaled_height = icon_height * scale_factor;

    let icon_x = icon_pos.x - (scaled_width / 2.0);
    let icon_y = icon_pos.y - (scaled_height / 2.0);

    let source_rect = Rectangle::new(0.0, 0.0, icon_width, icon_height);
    let dest_rect = Rectangle::new(icon_x, icon_y, scaled_width, scaled_height);

    d.draw_texture_pro(
        icon,
        source_rect,
        dest_rect,
        Vector2::new(0.0, 0.0),
        0.0,
        Color::WHITE,
    );
}
