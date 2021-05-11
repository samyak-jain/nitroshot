use gdk_pixbuf::Pixbuf;
use gio::{Cancellable, MemoryInputStream};
use glib::Bytes;
use gtk::prelude::*;

static SELECTION: &[u8] = include_bytes!("../../assets/selection.png");
static SCREEN: &[u8] = include_bytes!("../../assets/screen.png");
static WINDOW: &[u8] = include_bytes!("../../assets/window.png");

pub enum Icon {
    Selection,
    Screen,
    Window,
}

pub fn generate_image(icon_type: Icon) -> gtk::Image {
    let bytes = match icon_type {
        Icon::Selection => Bytes::from_static(SELECTION),
        Icon::Screen => Bytes::from_static(SCREEN),
        Icon::Window => Bytes::from_static(WINDOW),
    };
    let data_stream = MemoryInputStream::from_bytes(&bytes);
    let pixbuf = Pixbuf::from_stream(&data_stream, None as Option<&Cancellable>).unwrap();
    let image = gtk::Image::from_pixbuf(Some(&pixbuf));
    image.set_margin_top(5);
    image.set_margin_bottom(5);
    image
}
