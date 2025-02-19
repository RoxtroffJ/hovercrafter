//! User Interface module. It provides all the functions to build the UI, and link it to the physics engine

use relm4::{gtk, ComponentParts, RelmApp, RelmWidgetExt, SimpleComponent};
use relm4::gtk::prelude::*;

/// The type of the UI. 
pub struct UI {
    name: &'static str,
    label: &'static str
}

struct UIModel {
    label: &'static str
}

#[relm4::component]
impl SimpleComponent for UIModel {
    type Init = UI;
    type Input = ();
    type Output = ();

    view! {
        #[name = "app_window"]
        gtk::ApplicationWindow {
            gtk::Label {
                set_margin_all: 5,
                #[watch]
                set_label: model.label
            }
        }
    }

    fn init(
            init: Self::Init,
            root: Self::Root,
            _sender: relm4::ComponentSender<Self>,
        ) -> relm4::ComponentParts<Self> {
        let model = UIModel{label: init.label};
        let widgets = view_output!();
        
        widgets.app_window.set_title(Some(init.name));

        ComponentParts {model, widgets}
    }
}

impl UI {
    /// Creates a new UI. 
    /// 
    /// The app is named `name`, and displays the `label`.
    pub fn new(name: &'static str, label: &'static str) -> Self {
        Self {name, label}
    }

    /// Starts the UI. It is identified by the OS with the given `id`.
    pub fn run(self, id: &'static str) {
        let app = RelmApp::new(id);
        app.run::<UIModel>(self)
    }
}