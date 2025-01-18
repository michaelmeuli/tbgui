mod tbgui;
mod utils;
mod views;
mod types;
mod config;

use tbgui::Tbgui;

const RESULT_DIR_LOCAL: &str = "tb-profiler-results";
const DEFAULT_TEMPLATE_FILENAME_LOCAL: &str = "default_template.docx";
const USER_TEMPLATE_FILENAME_LOCAL: &str = "user_template.docx";

pub fn main() -> iced::Result {
    tracing_subscriber::fmt::init();

    iced::application(Tbgui::title, Tbgui::update, Tbgui::view)
        .subscription(Tbgui::subscription)
        .font(include_bytes!("../fonts/icons.ttf").as_slice())
        .window_size((570.0, 800.0))
        .run_with(Tbgui::new)
}
