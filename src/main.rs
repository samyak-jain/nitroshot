use crate::screenshot::select::select_screen;

mod screenshot;

sixtyfps::include_modules!();
fn main() {
    let window = MainWindow::new();

    let weak_window = window.as_weak();

    window.on_start_recording(|capture_type, delay| {
        println!("Capture Type: {}; Delay: {}", capture_type, delay);
    });

    window.on_take_screenshot(move |capture_type, delay| {
        println!("Capture Type: {}; Delay: {}", capture_type, delay);

        if let Some(strong_window) = weak_window.upgrade() {
            strong_window.hide();
            let res = select_screen();

            println!("Selected area: {}", res);

            strong_window.show();
        }
    });

    window.run();
}
