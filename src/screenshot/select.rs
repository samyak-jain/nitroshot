use hacksaw::HackSawConfig;

pub fn select_screen() {
    let config = HackSawConfig::default();
    hacksaw::launch_default(Some(config), false).unwrap();
}
