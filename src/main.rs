use crate::screenshot::select::select_screen;

mod screenshot;

#[derive(Debug)]
enum CaptureType {
    WINDOW,
    SCREEN,
    SELECTION,
    NONE
}

impl From<i32> for CaptureType {
    fn from(number: i32) -> Self {
        match number {
            0 => Self::SELECTION,
            1 => Self::SCREEN,
            2 => Self::WINDOW,
            _ => Self::NONE,
        }
    }
}

sixtyfps::include_modules!();

fn main() {
    let window = MainWindow::new();

    let weak_window = window.as_weak();

    window.global::<AppState>().on_start_recording(|| {
        println!("recording")
    });

    window.global::<AppState>().on_take_screenshot(|| {
        if let Some(strong_window) = weak_window.upgrade() {
            let capture_type = CaptureType::from(strong_window.global::<AppState>().get_capture_type());
            let delay = strong_window.global::<AppState>().get_delay();

            println!("Capture Type: {:#?}; Delay: {}", capture_type, delay);
            strong_window.hide();

            select_screen(capture_type, delay);

            strong_window.show();
        }
    });

    window.run();
}
