use relm::Relm;
use relm::Widget;
use relm_derive::widget;
use relm_derive::Msg;

pub struct Model {}

#[derive(Msg)]
pub enum Msg {
    Quit,
}

#[widget]
impl Widget for StyledButton {
    fn init_view(&mut self) {
        use gtk::{CssProviderExt, StyleContextExt, WidgetExt};
        // Adjust the look of the entry.
        let style_context = self.widgets.button.get_style_context();
        let style = include_bytes!("../../style/button.css");
        let provider = gtk::CssProvider::new();
        provider.load_from_data(style).unwrap();
        style_context.add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
    }

    fn model(_relm: &Relm<Self>, _: ()) -> Model {
        Model {}
    }

    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::Quit => gtk::main_quit(),
        }
    }

    view! {
        #[name="button"]
        gtk::Button {},
    }
}
