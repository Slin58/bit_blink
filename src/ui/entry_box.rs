use libadwaita::ActionRow;
use libadwaita::glib::IsA;
use libadwaita::gtk::{Box, CenterBox, Entry};
use libadwaita::gtk::Widget;
use libadwaita::prelude::{ActionRowExt, BoxExt, WidgetExt};

pub fn build_entry_box() -> (impl IsA<Widget>, Entry) {
    let entry_box = CenterBox::builder().margin_top(15).margin_bottom(15).margin_start(5).margin_end(5).build();
    let entry = Entry::builder().can_focus(true).focus_on_click(true).margin_top(5).margin_end(5).margin_bottom(5).margin_start(2).hexpand(false).vexpand(true).placeholder_text("Type your Text here...").build();
    entry_box.set_center_widget(Some(&entry));
    (entry_box, entry)
}