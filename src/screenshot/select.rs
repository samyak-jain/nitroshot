use hacksaw::HackSawConfig;

#[derive(Debug)]
pub enum CaptureType {
    WINDOW,
    SCREEN,
    SELECTION,
    NONE,
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

pub fn select_screen(capture_type: CaptureType, delay: i32) {
    let config = HackSawConfig::default();

    match capture_type {
        CaptureType::WINDOW => todo!(),
        CaptureType::SCREEN => todo!(),
        CaptureType::SELECTION => {
            let hacksaw_result = hacksaw::make_selection(Some(config)).unwrap();
        }
        CaptureType::NONE => return,
    }
}
