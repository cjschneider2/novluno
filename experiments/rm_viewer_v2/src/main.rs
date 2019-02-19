#![allow(unused_imports, dead_code)]

extern crate gtk;
#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;

use gtk::prelude::*;
use gtk::{Window, Inhibit, WindowType};
use relm::{Relm, Update, Widget};

struct Model {}

#[derive(Msg)]
enum Msg {
    Quit,
}

struct Win {
    model: Model,
    window: Window,
}

impl Update for Win {
    type Model = Model;
    type ModelParam = ();
    type Msg = Msg;

    fn model(_: &Relm<Self>, _: ()) -> Model {
        Model {}
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::Quit => gtk::main_quit(),
        }
    }
}

impl Widget for Win {
    type Root = Window;

    fn root(&self) -> Self::Root {
        self.window.clone()
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        let window = Window::new(WindowType::Toplevel);

        connect!(relm, window, connect_delete_event(_,_), return (Some(Msg::Quit), Inhibit(false)));

        window.show_all();

        Win {
            model,
            window,
        }
    }
}

fn main() {
    Win::run(()).unwrap();
}
