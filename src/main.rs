mod config;
mod ssh;
mod tbgui;
mod types;
mod utils;
mod views;

use tbgui::Tbgui;

const RESULT_DIR_LOCAL: &str = "tb-profiler-results";
const DEFAULT_TEMPLATE_FILENAME_LOCAL: &str = "default_template.docx";

pub fn main() -> iced::Result {
    tracing_subscriber::fmt::init();

    iced::application(Tbgui::title, Tbgui::update, Tbgui::view)
        .subscription(Tbgui::subscription)
        .font(include_bytes!("../fonts/icons.ttf").as_slice())
        .window_size((570.0, 800.0))
        .run_with(Tbgui::new)
}
