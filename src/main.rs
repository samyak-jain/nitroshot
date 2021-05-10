use gtk::prelude::*;
use gtk::Inhibit;
use gtk::Orientation::{Horizontal, Vertical};
use gtk::Align;
use relm::Widget;
use relm_derive::widget;
use glib::Bytes;
use relm_derive::Msg;
use gio::{Cancellable, MemoryInputStream};
use gdk_pixbuf::Pixbuf;

pub struct Model {
    counter: u32,
}

#[derive(Msg)]
pub enum Msg {
    Decrement,
    Increment,
    Quit,
}

static SELECTION: &[u8] = include_bytes!("../assets/selection.png");
static SCREEN: &[u8] = include_bytes!("../assets/screen.png");
static WINDOW: &[u8] = include_bytes!("../assets/window.png");

enum Icon {
    Selection,
    Screen,
    Window,
}

fn generate_image(icon_type: Icon) -> gtk::Image {
    let bytes = match icon_type {
        Icon::Selection => Bytes::from_static(SELECTION),
        Icon::Screen => Bytes::from_static(SCREEN),
        Icon::Window => Bytes::from_static(WINDOW),
    };
    let data_stream = MemoryInputStream::from_bytes(&bytes);
    let pixbuf = Pixbuf::from_stream(&data_stream, None as Option<&Cancellable>).unwrap();
    let image = gtk::Image::from_pixbuf(Some(&pixbuf));
    image
}

#[widget]
impl Widget for Win {
    fn model() -> Model {
        Model { counter: 0 }
    }

    fn update(&mut self, event: Msg) {
        match event {
            // A call to self.label1.set_text() is automatically inserted by the
            // attribute every time the model.counter attribute is updated.
            Msg::Decrement => self.model.counter -= 1,
            Msg::Increment => self.model.counter += 1,
            Msg::Quit => gtk::main_quit(),
        }
    }

    view! {
        gtk::Window {
            gtk::Box {
                orientation: Vertical,
                valign: Align::Center,
                homogeneous: true,
                gtk::Box {
                    halign: Align::Center,
                    orientation: Horizontal,
                    gtk::Button {
                        clicked => Msg::Increment,
                        property_margin: 20,
                        label: "Take Screenshot",
                    },
                    gtk::Button {
                        clicked => Msg::Increment,
                        property_margin: 20,
                        label: "Start Recording",
                    },
                },
                gtk::Box {
                    orientation: Horizontal,
                    halign: Align::Center,
                    gtk::Button {
                        clicked => Msg::Increment,
                        label: "Selection",
                        child: {
                            padding: 10,
                        },
                        always_show_image: true,
                        image: Some(&generate_image(Icon::Selection)),
                        image_position: gtk::PositionType::Top,
                    },
                    gtk::Button {
                        clicked => Msg::Increment,
                        label: "Screen",
                        child: {
                            padding: 10,
                        },
                        always_show_image: true,
                        image: Some(&generate_image(Icon::Screen)),
                        image_position: gtk::PositionType::Top,
                    },
                    gtk::Button {
                        clicked => Msg::Increment,
                        label: "Window",
                        child: {
                            padding: 10,
                        },
                        always_show_image: true,
                        image: Some(&generate_image(Icon::Window)),
                        image_position: gtk::PositionType::Top,
                    },
                },
                gtk::Box {
                    orientation: Horizontal,
                    homogeneous: true,
                    gtk::Label {
                        halign: Align::Start,
                        property_margin: 20,
                        text: "Delay in Seconds",
                    },
                    gtk::Box {
                        orientation: Horizontal,
                        halign: Align::End,
                        gtk::Button {
                            clicked => Msg::Decrement,
                            margin_end: 10,
                            margin_top: 20,
                            margin_bottom: 20,
                            label: "-",
                        },
                        gtk::Label {
                            text: &self.model.counter.to_string(),
                        },
                        gtk::Button {
                            clicked => Msg::Increment,
                            margin_start: 10,
                            margin_top: 20,
                            margin_bottom: 20,
                            margin_end: 20,
                            label: "+",
                        },
                    },
                },
            },
            // Use a tuple when you want to both send a message and return a value to
            // the GTK+ callback.
            delete_event(_, _) => (Msg::Quit, Inhibit(false)),
        }
    }
}

fn main() {
    Win::run(()).unwrap();
}
