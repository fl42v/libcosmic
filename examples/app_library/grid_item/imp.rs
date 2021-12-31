use std::cell::Cell;

use gtk4::glib;
use gtk4::prelude::*;
use gtk4::subclass::prelude::*;
use gtk4::CompositeTemplate;

#[derive(Debug, Default, CompositeTemplate)]
#[template(file = "grid_item.ui")]
pub struct GridItem {
    #[template_child]
    pub name: TemplateChild<gtk4::Label>,
    #[template_child]
    pub image: TemplateChild<gtk4::Image>,
    pub index: Cell<u32>,
}

#[glib::object_subclass]
impl ObjectSubclass for GridItem {
    const NAME: &'static str = "GridItem";
    type Type = super::GridItem;
    type ParentType = gtk4::Box;

    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass);
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for GridItem {}

impl WidgetImpl for GridItem {}

impl BoxImpl for GridItem {}