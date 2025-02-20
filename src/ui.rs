//! The graphics library of the app.
//! It is based on [relm4] (which itself uses [gtk4](relm4::gtk)).
//! It defines all the components, and all the app structure.

use relm4::{gtk::{self, prelude::GtkWindowExt}, ComponentParts, RelmApp, SimpleComponent};


/// All the standalone components are defined in this module.
pub mod components;
use components::*;
/// The app's pages are defined in this module.
pub mod pages;
// use pages::*;

/// Type for the whole app. 
pub struct UI {
    title: &'static str,
    margins: u8
}

struct AppModel;

#[relm4::component]
impl SimpleComponent for AppModel {
    type Init = UI;
    type Input = ();
    type Output = ();

    view! {
        #[root]
        gtk::ApplicationWindow {

            set_title: Some(title),

            #[template]
            PageTab(margins) {
                #[template_child]
                stack {
                    #[template]
                    add_titled[Some("motorisatiob_page"), "Motorisation"] = &Placeholder(margins) {
                        set_label: "Motorisation"
                    },

                    #[template]
                    add_titled[Some("levitation_page"), "Levitation"] = &Placeholder(margins) {
                        set_label: "Levitation"
                    },

                    #[template]
                    add_titled[Some("aero_page"), "Aerodynamics"] = &Placeholder(margins) {
                        set_label: "Aerodynamics"
                    },

                    #[template]
                    add_titled[Some("recap_page"), "Recap"] = &Placeholder(margins) {
                        set_label: "Recap"
                    }
                }
            }
        }
    }

    fn init(
            init: Self::Init,
            root: Self::Root,
            _sender: relm4::ComponentSender<Self>,
        ) -> relm4::ComponentParts<Self> {
        let margins= init.margins.into();
        let title = init.title;
        
        let model = AppModel;
        let widgets = view_output!();

        ComponentParts {model, widgets}
    }
}

impl UI {
    /// Creates a new UI
    pub fn new(title: &'static str, margins: u8) -> Self {
        Self { title, margins }
    }

    /// Runs the UI with the given ID
    pub fn run(self, id: &'static str) {
        let app = RelmApp::new(id);
        app.run::<AppModel>(self);
    }
}