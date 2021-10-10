use crate::screenshot::select::{select_screen, CaptureType};

mod screenshot;

sixtyfps::include_modules!();

fn main() {
    let window = MainWindow::new();

    let weak_window = window.as_weak();

    window
        .global::<AppState>()
        .on_start_recording(|| println!("recording"));

    window.global::<AppState>().on_take_screenshot(move || {
        if let Some(strong_window) = weak_window.upgrade() {
            let capture_type =
                CaptureType::from(strong_window.global::<AppState>().get_capture_type());
            let delay = strong_window.global::<AppState>().get_delay();

            println!("Capture Type: {:#?}; Delay: {}", capture_type, delay);
            strong_window.hide();

            select_screen(capture_type, delay);

            strong_window.show();
        }
    });

    window.run();
}
