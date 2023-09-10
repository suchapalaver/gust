use colored::Colorize;
use common::helpers::ItemInfo;

use crate::models::Section;
pub(crate) fn display<T: ItemInfo>(items: Vec<T>, to_display: &str) {
    println!("{}{}", to_display.blue().bold(), ":".blue().bold());
    for item in items {
        println!(" {} {}", "-".bold().blue(), item.name().blue());
    }
}

pub(crate) fn display_sections(items: Vec<Section>, to_display: &str) {
    display(items, to_display)
}
