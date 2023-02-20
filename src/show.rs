use colored::Colorize;

use crate::{models::Section, ItemInfo};
pub(crate) fn display<T: ItemInfo>(items: Vec<T>, to_display: &str) {
    println!("{}{}", to_display.blue().bold(), ":".blue().bold());
    for item in items {
        println!(" {} {}", "-".bold().blue(), item.name().blue());
    }
}

pub(crate) fn display_sections(items: Vec<Section>, to_display: &str) {
    display(items, to_display)
}
