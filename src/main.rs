use gtk::prelude::*;
use gtk::cairo;
use gtk::{Application, ApplicationWindow, DrawingArea};

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

fn draw_circle(_: &DrawingArea, ctx: &cairo::Context, width: i32, height: i32) {
    ctx.set_operator(cairo::Operator::Clear);
    ctx.paint().unwrap();
    ctx.set_operator(cairo::Operator::Over);

    let outer_radius = width.min(height) as f64 / 4.0;
    let inner_radius = outer_radius / 2.0;
    let center_x = width as f64 / 2.0;
    let center_y = height as f64 / 2.0;

    ctx.set_source_rgba(1.0, 0.0, 0.5, 0.3);
    ctx.new_path();
    ctx.arc(center_x, center_y, outer_radius, 0.0, std::f64::consts::TAU);
    ctx.arc_negative(center_x, center_y, inner_radius, 0.0, -std::f64::consts::TAU);
    ctx.fill().unwrap();
}

fn build_circle(app: &Application) {
    let area = DrawingArea::new();
    area.set_draw_func(draw_circle);

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
