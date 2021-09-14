use glib::SignalHandlerId;
use gtk::prelude::*;
use gtk::Align;
use gtk::Inhibit;
use gtk::Orientation::{Horizontal, Vertical};
use hacksaw::launch_default;
use relm::{Relm, Widget};
use relm_derive::widget;
use relm_derive::Msg;

mod components;
use components::image::generate_image;
use components::image::Icon;

pub struct Model {
    counter: u32,    selection: String,
    active_state: Activity,
    relm: Relm<Win>,
    selection_handle: Option<SignalHandlerId>,
    screen_handle: Option<SignalHandlerId>,
    window_handle: Option<SignalHandlerId>,
}

#[derive(Debug)]
pub enum Activity {
    Selection,
    Screen,
    Window,
    None,
}

#[derive(Msg)]
pub enum Msg {
    Screenshot,
    Selection,
    SelectionActive,
    ScreenActive,
    WindowActive,
    Decrement,
    Increment,
    Quit,
}

#[widget]
impl Widget for Win {
    fn model(relm: &Relm<Self>, _: ()) -> Model {
        Model {
            counter: 0,
            selection: String::default(),
            active_state: Activity::None,
            relm: relm.clone(),
            selection_handle: None,
            screen_handle: None,
            window_handle: None,
        }
    }

    fn init_view(&mut self) {
        let selection_stream = self.model.relm.stream().clone();
        self.model.selection_handle = Some(
            self.widgets
                .selection
                .connect_clicked(move |_| selection_stream.emit(Msg::SelectionActive)),
        );

        let screen_stream = self.model.relm.stream().clone();
        self.model.screen_handle = Some(
            self.widgets
                .screen
                .connect_clicked(move |_| screen_stream.emit(Msg::ScreenActive)),
        );

        let window_stream = self.model.relm.stream().clone();
        self.model.window_handle = Some(
            self.widgets
                .window
                .connect_clicked(move |_| window_stream.emit(Msg::WindowActive)),
        );
    }

    fn update(&mut self, event: Msg) {
        if let Some(handler) = &self.model.selection_handle {
            self.widgets.selection.block_signal(handler);
        }
        if let Some(handler) = &self.model.screen_handle {
            self.widgets.screen.block_signal(handler);
        }
        if let Some(handler) = &self.model.window_handle {
            self.widgets.window.block_signal(handler);
        }

        match event {
            Msg::Decrement => self.model.counter -= 1,
            Msg::Increment => self.model.counter += 1,
            Msg::Quit => gtk::main_quit(),
            Msg::SelectionActive => {
                match self.model.active_state {
                    Activity::Selection => {
                        self.model.active_state = Activity::None;
                    }
                    Activity::None => {
                        self.model.active_state = Activity::Selection;
                    }
                    Activity::Screen => {
                        self.widgets.screen.set_active(false);
                        self.model.active_state = Activity::Selection;
                    }
                    Activity::Window => {
                        self.widgets.window.set_active(false);
                        self.model.active_state = Activity::Selection;
                    }
                };
            }
            Msg::ScreenActive => {
                match self.model.active_state {
                    Activity::Selection => {
                        self.widgets.selection.set_active(false);
                        self.model.active_state = Activity::Screen;
                    }
                    Activity::None => {
                        self.model.active_state = Activity::Screen;
                    }
                    Activity::Screen => {
                        self.model.active_state = Activity::None;
                    }
                    Activity::Window => {
                        self.widgets.window.set_active(false);
                        self.model.active_state = Activity::Screen;
                    }
                };
            }
            Msg::WindowActive => {
                match self.model.active_state {
                    Activity::Selection => {
                        self.widgets.selection.set_active(false);
                        self.model.active_state = Activity::Window;
                    }
                    Activity::None => {
                        self.model.active_state = Activity::Window;
                    }
                    Activity::Screen => {
                        self.widgets.screen.set_active(false);
                        self.model.active_state = Activity::Window;
                    }
                    Activity::Window => {
                        self.model.active_state = Activity::None;
                    }
                };
            }
            Msg::Selection => {
                let result = launch_default(None, false);
                // self.widgets.main_window.show();
                match result {
                    Ok(selection) => {
                        self.model.selection = selection;
                    }
                    Err(err) => {
                        println!("{}", err);
                    }
                };

                self.widgets.selection.set_active(false);
            }
            Msg::Screenshot => {
                match self.model.active_state {
                    Activity::Selection => {
                        self.widgets.main_window.get_toplevel().unwrap().hide();
                        self.model.relm.stream().emit(Msg::Selection);
                    }
                    _ => (),
                };

                self.model.active_state = Activity::None;
            }
        };

        if let Some(handler) = &self.model.selection_handle {
            self.widgets.selection.unblock_signal(handler);
        }
        if let Some(handler) = &self.model.screen_handle {
            self.widgets.screen.unblock_signal(handler);
        }
        if let Some(handler) = &self.model.window_handle {
            self.widgets.window.unblock_signal(handler);
        }
    }

    view! {
        #[name = "main_window"]
        gtk::Window {
            gtk::Box {
                orientation: Vertical,
                valign: Align::Center,
                homogeneous: true,
                gtk::Box {
                    halign: Align::Center,
                    orientation: Horizontal,
                    gtk::Button {
                        clicked => Msg::Screenshot,
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
                    #[name = "selection"]
                    gtk::ToggleButton {
                        label: "Selection",
                        child: {
                            padding: 10,
                        },
                        always_show_image: true,
                        image: Some(&generate_image(Icon::Selection)),
                        image_position: gtk::PositionType::Top,
                    },
                    #[name = "screen"]
                    gtk::ToggleButton {
                        label: "Screen",
                        child: {
                            padding: 10,
                        },
                        always_show_image: true,
                        image: Some(&generate_image(Icon::Screen)),
                        image_position: gtk::PositionType::Top,
                    },
                    #[name = "window"]
                    gtk::ToggleButton {
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
