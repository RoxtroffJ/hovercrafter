use relm4::{gtk, ComponentParts, RelmApp, RelmWidgetExt, SimpleComponent};
use relm4::gtk::prelude::*;

#[derive(Debug)]
struct AppModel {
    label: & 'static str
}

#[relm4::component]
impl SimpleComponent for AppModel {
    type Init = &'static str;
    type Input = ();
    type Output = ();

    view! {
        gtk::ApplicationWindow {
            set_title: Some("Hovercrafter"),

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
        let model = AppModel {label: init};
        let widgets = view_output!();

        ComponentParts {model, widgets}
    }
}

fn main() {
    let app = RelmApp::new("AJ.predim.hovercrafter");
    app.run::<AppModel>("Welcome! Let's make some hovercrafts!")
}
