pub mod home;
pub mod library;
pub mod reader;
pub mod settings;


use eframe::egui::Context;

use crate::app::{MangaApp, Screen};

pub fn show(ctx: &Context, app: &mut MangaApp) {
    match app.current_page {
        Screen::Home => home::show(ctx, app),
        Screen::Library => library::show(ctx, app), 
        Screen::Reader => reader::show(ctx, app),
        Screen::Settings => println!("also not"),//settings::show(ctx, app),
    }
}