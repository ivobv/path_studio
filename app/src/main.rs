slint::include_modules!();

fn main() {
    let app = AppWindow::new().unwrap();
    app.window().set_maximized(true);
    app.run().unwrap();
}

