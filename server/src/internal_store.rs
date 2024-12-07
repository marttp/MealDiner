use crate::menu::model::Menu;
use once_cell::sync::Lazy;
use std::sync::Arc;

pub static MENUS: Lazy<Arc<[Menu]>> = Lazy::new(|| {
    Arc::new([
        Menu::new("Ramen"),
        Menu::new("Beef rice"),
        Menu::new("Beer"),
    ])
});
