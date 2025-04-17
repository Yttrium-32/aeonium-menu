use gtk::prelude::*;
use gtk::cairo;
use gtk::{Application, ApplicationWindow, DrawingArea};

const APP_ID: &str = "org.example.test";

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

    let radius = width.min(height) as f64 / 3.0;
    let center_x = width as f64 / 2.0;
    let center_y = height as f64 / 2.0;

    ctx.set_source_rgba(1.0, 0.0, 0.5, 0.3);
    ctx.arc(center_x, center_y, radius, 0.0, std::f64::consts::TAU);
    ctx.fill().unwrap();
}

fn build_circle(app: &Application) {
    let area = DrawingArea::new();
    area.set_draw_func(draw_circle);

    let win = ApplicationWindow::builder()
        .application(app)
        .decorated(false)
        .resizable(false)
        .title("Test")
        .child(&area)
        .default_width(400)
        .default_height(400)
        .build();

    win.present();
}
