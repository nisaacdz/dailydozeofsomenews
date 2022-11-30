use eframe::{epaint::Vec2, NativeOptions};
use lib::api::Api;
use ui::UI;

mod ui;

fn main() {
    let api = Api::new();
    let mut win_ops = NativeOptions::default();

    win_ops.initial_window_size = Some(Vec2::new(420., 640.));

    eframe::run_native(
        "DailyDozeOfSomeNews",
        win_ops,
        Box::new(|cc| Box::new(UI::new(cc, api))),
    );
}
