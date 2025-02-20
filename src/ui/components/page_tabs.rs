//! This components allows group pages under some tabs.

use std::ops::Deref;

use relm4::{gtk::{self, prelude::{BoxExt, WidgetExt}}, WidgetTemplate};

/// Template for a page tab group
#[derive(Debug)]
pub struct PageTab {
    root: gtk::Box,
    /// The [gtk::Stack] in which the pages are.
    pub stack: gtk::Stack
}

impl WidgetTemplate for PageTab {
    type Root = gtk::Box;
    type Init = i32;

    fn init(spacing: Self::Init) -> Self {
        let stack = gtk::Stack::new();
        let stack_switcher = gtk::StackSwitcher::new();

        stack.set_transition_type(gtk::StackTransitionType::SlideLeftRight);
        stack.set_widget_name("tab_stack");

        stack_switcher.set_stack(Some(&stack));

        let root = gtk::Box::new(gtk::Orientation::Vertical, spacing);
        root.append(&stack_switcher);
        root.append(&stack);
        
        Self {root, stack}
    }
}

impl Deref for PageTab {
    type Target = gtk::Box;

    fn deref(&self) -> &Self::Target {
        &self.root
    }
}

impl AsRef<gtk::Box> for PageTab {
    fn as_ref(&self) -> &gtk::Box {
        &self.root
    }
}