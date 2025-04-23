use gtk::prelude::*;
use gtk::cairo;
use gtk::{Application, ApplicationWindow, DrawingArea};

use std::path::Path;
use std::f64::consts::{ TAU, PI };

const WIN_W: i32 = 1920;
const WIN_H: i32 = 1080;

const APP_ID: &str = "yttrium32.aeonium.menu";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_startup(|_| { load_css() });
    app.connect_activate(build_circle);
    app.run();
}

fn load_css() {
    let provider = gtk::CssProvider::new();
    provider.load_from_string("
        window {
            background-color: transparent;
        }
    ");

    gtk::style_context_add_provider_for_display(
        &gtk::gdk::Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION
    );
}

fn draw_arrowhead(ctx: &cairo::Context, center_x: f64, center_y: f64, radius: f64, angle_deg: f64, size: f64) {
    let offset_distance = 10.0;

    let angle = (angle_deg - 90.0) * PI / 180.0;

    let adjusted_radius = radius - offset_distance;

    let tip_x = center_x + adjusted_radius * angle.cos();
    let tip_y = center_y + adjusted_radius * angle.sin();

    let left_x = center_x + (adjusted_radius - size) * angle.cos() - (size / 2.0) * angle.sin();
    let left_y = center_y + (adjusted_radius - size) * angle.sin() + (size / 2.0) * angle.cos();

    let right_x = center_x + (adjusted_radius - size) * angle.cos() + (size / 2.0) * angle.sin();
    let right_y = center_y + (adjusted_radius - size) * angle.sin() - (size / 2.0) * angle.cos();

    ctx.set_source_rgba(0.0, 0.0, 0.0, 1.0);
    ctx.set_line_width(4.0);

    ctx.new_path();

    ctx.move_to(left_x, left_y);
    ctx.line_to(tip_x, tip_y);

    ctx.move_to(tip_x, tip_y);
    ctx.line_to(right_x, right_y);

    ctx.stroke().unwrap();
}

fn draw_circle(ctx: &cairo::Context, width: i32, height: i32, radius: f64) {
    ctx.set_operator(cairo::Operator::Clear);
    ctx.paint().unwrap();
    ctx.set_operator(cairo::Operator::Over);

    let outer_radius = radius;
    let inner_radius = outer_radius / 1.8;

    let center_x = width as f64 / 2.0;
    let center_y = height as f64 / 2.0;

    ctx.set_source_rgba(1.0, 0.0, 0.5, 0.3);
    ctx.new_path();
    ctx.arc(center_x, center_y, outer_radius, 0.0, TAU);
    ctx.arc_negative(center_x, center_y, inner_radius, 0.0, -TAU);
    ctx.fill().unwrap();

    draw_arrowhead(ctx, center_x, center_y, inner_radius, 45.0, 50.0);
}

fn build_circle(app: &Application) {
    let area = DrawingArea::new();

    area.set_draw_func(move |_, ctx, w, h| {
        let radius = w.min(h) as f64 / 5.0;
        draw_circle(ctx, w, h, radius);
    });

    let win = ApplicationWindow::builder()
        .application(app)
        .decorated(false)
        .resizable(false)
        .child(&area)
        .default_width(WIN_W)
        .default_height(WIN_H)
        .build();

    win.fullscreen();
    win.present();
}

