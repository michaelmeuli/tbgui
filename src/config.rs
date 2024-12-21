use serde::{Deserialize, Serialize};

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
            username: "mimeul".into(),
            remote_raw_dir: "/shares/sander.imm.uzh/MM/PRJEB57919/raw".into(),
            tb_profiler_script: "/shares/sander.imm.uzh/MM/PRJEB57919/scripts/tbprofiler.sh".into(),
            remote_results_dir: "/shares/sander.imm.uzh/MM/PRJEB57919/out/results".into(),
            default_template_remote: "/shares/sander.imm.uzh/MM/PRJEB57919/tb-profiler-templates/docx/default_template.docx".into(),
            user_template_remote: "/shares/sander.imm.uzh/MM/PRJEB57919/template/user_template.docx".into(),
        }
    }
}


//const USER_TEMPLATE_REMOTE_DIR: &str = "/home/mimeul/shares/MM/PRJEB57919/template";

