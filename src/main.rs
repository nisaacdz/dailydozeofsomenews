mod api;
mod ui;

use api::NewsApi;
use dotenv::dotenv;
use eframe::{self, epaint::Vec2, NativeOptions};
use std::error::Error;
use ui::NewsUI;

fn main() -> Result<(), Box<dyn Error>> {
    dotenv()?;
    let api_key = std::env::var("API_KEY")?;
    let address = String::from("https://newsapi.org/v2");

    let api = NewsApi::new(address, api_key);
    let mut win_options = NativeOptions::default();
    win_options.initial_window_size = Some(Vec2::new(420., 640.));

    eframe::run_native(
        "DailyDozeOfSomeNews",
        win_options,
        Box::new(|cc| Box::new(NewsUI::new(cc, api))),
    );

    Ok(())
}
