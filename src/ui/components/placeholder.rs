//! This component is a development tool. It replaces a not yet implemented component

use std::ops::Deref;

use relm4::{gtk, RelmWidgetExt, WidgetTemplate};

/// Template for a placeholder Widget
#[derive(Debug)]
pub struct Placeholder {
    /// The label of the placeholder
    pub label: gtk::Label
}

impl WidgetTemplate for Placeholder {
    type Root = gtk::Label;
    type Init = i32;

    fn init(margin: Self::Init) -> Self {
        
        let label = gtk::Label::new(Some("Placeholder"));
        label.set_margin_all(margin);
        
        Self {label}
    }
}

impl Deref for Placeholder {
    type Target = gtk::Label;

    fn deref(&self) -> &Self::Target {
        &self.label
    }
}

impl AsRef<gtk::Label> for Placeholder {
    fn as_ref(&self) -> &gtk::Label {
        &self.label
    }
}