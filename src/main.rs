mod tbgui;
mod utils;
mod views;
mod types;

use iced;
use tbgui::Tbgui;

const USERNAME: &str = "mimeul";
const REMOTE_RAW_DIR: &str = "/shares/sander.imm.uzh/MM/PRJEB57919/raw";
const TB_PROFILER_SCRIPT: &str = "/shares/sander.imm.uzh/MM/PRJEB57919/scripts/tbprofiler.sh";
const REMOTE_RESULTS_DIR: &str = "/shares/sander.imm.uzh/MM/PRJEB57919/out/results";

pub fn main() -> iced::Result {
    tracing_subscriber::fmt::init();

    iced::application(Tbgui::title, Tbgui::update, Tbgui::view)
        .subscription(Tbgui::subscription)
        .font(include_bytes!("../fonts/icons.ttf").as_slice())
        .window_size((570.0, 800.0))
        .run_with(Tbgui::new)
}
