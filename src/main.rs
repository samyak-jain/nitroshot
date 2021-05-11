use gtk::prelude::*;
use gtk::Align;
use gtk::Inhibit;
use gtk::Orientation::{Horizontal, Vertical};
use relm::Widget;
use relm_derive::widget;
use relm_derive::Msg;

mod components;
use components::image::generate_image;
use components::image::Icon;

pub struct Model {
    counter: u32,
}

#[derive(Msg)]
pub enum Msg {
    Decrement,
    Increment,
    Quit,
}


#[widget]
impl Widget for Win {
    fn model() -> Model {
        Model { counter: 0 }
    }

    fn update(&mut self, event: Msg) {
        match event {
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
