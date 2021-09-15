use hacksaw::HackSawConfig;

pub fn select_screen() -> String {
    let config = HackSawConfig::default();
    let hacksaw_result = hacksaw::make_selection(Some(config)).unwrap();
}
