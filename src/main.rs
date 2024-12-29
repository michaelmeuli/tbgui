mod tbgui;
mod utils;
mod views;
mod types;
mod config;

use iced;
use tbgui::Tbgui;

const TB_PROFILER_SCRIPT: &str = "/shares/sander.imm.uzh/MM/PRJEB57919/scripts/tbprofiler.sh";
const USER_TEMPLATE_REMOTE: &str = "/shares/sander.imm.uzh/MM/PRJEB57919/template/user_template.docx";


pub fn main() -> iced::Result {
    tracing_subscriber::fmt::init();

    iced::application(Tbgui::title, Tbgui::update, Tbgui::view)
        .subscription(Tbgui::subscription)
        .font(include_bytes!("../fonts/icons.ttf").as_slice())
        .window_size((570.0, 800.0))
        .run_with(Tbgui::new)
}
