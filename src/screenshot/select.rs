use hacksaw::HackSawConfig;

use crate::CaptureType;

pub fn select_screen(capture_type: CaptureType, delay: i32) {
    match capture_type {
        CaptureType::WINDOW => todo!(),
        CaptureType::SCREEN => todo!(),
        CaptureType::SELECTION => todo!(),
        CaptureType::NONE => return,
    }

    let config = HackSawConfig::default();
    let hacksaw_result = hacksaw::make_selection(Some(config)).unwrap();
}
