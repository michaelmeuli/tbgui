use serde::{Deserialize, Serialize};

pub const TBGUI_USERNAME: &str = default_env(option_env!("TBGUI_USERNAME"), "mimeul");
pub const REMOTE_RAW_DIR: &str = default_env(
    option_env!("REMOTE_RAW_DIR"),
    "/shares/sander.imm.uzh/MM/PRJEB57919/raw",
);
pub const TB_PROFILER_SCRIPT: &str = default_env(
    option_env!("TB_PROFILER_SCRIPT"),
    "/shares/sander.imm.uzh/MM/PRJEB57919/scripts/tbprofiler.sh",
);
pub const REMOTE_RESULTS_DIR: &str = default_env(
    option_env!("REMOTE_RESULTS_DIR"),
    "/shares/sander.imm.uzh/MM/PRJEB57919/out/results",
);
pub const DEFAULT_TEMPLATE_REMOTE: &str = default_env(
    option_env!("DEFAULT_TEMPLATE_REMOTE"),
    "/shares/sander.imm.uzh/MM/PRJEB57919/tb-profiler-templates/docx/default_template.docx",
);
pub const USER_TEMPLATE_REMOTE: &str = default_env(
    option_env!("USER_TEMPLATE_REMOTE"),
    "/shares/sander.imm.uzh/MM/PRJEB57919/template/user_template.docx",
);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TbguiConfig {
    pub username: String,
    pub remote_raw_dir: String,
    pub tb_profiler_script: String,
    pub remote_results_dir: String,
    pub default_template_remote: String,
    pub user_template_remote: String,
}

impl ::std::default::Default for TbguiConfig {
    fn default() -> Self {
        Self {
            username: TBGUI_USERNAME.into(),
            remote_raw_dir: REMOTE_RAW_DIR.into(),
            tb_profiler_script: TB_PROFILER_SCRIPT.into(),
            remote_results_dir: REMOTE_RESULTS_DIR.into(),
            default_template_remote: DEFAULT_TEMPLATE_REMOTE.into(),
            user_template_remote: USER_TEMPLATE_REMOTE.into(),
        }
    }
}

const fn default_env(v: Option<&'static str>, default: &'static str) -> &'static str {
    match v {
        Some(v) => v,
        None => default,
    }
}
