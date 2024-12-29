use super::settings::Settings;
use std::{
    path::Path,
    process::{Command, Stdio},
};

pub fn spawn_single_command(cmd_param: &str, st: &Settings, exe_dir: &Path, bat: &Path) {
    println!("spawn => {}, number={}", cmd_param, st.number);
    Command::new("cmd")
        .env("ROOTFOLDER", exe_dir.to_string_lossy().to_string())
        .env("RC_PATH", &st.rc_path)
        .env("PREFIX", &st.prefix)
        .env("MARGIN", st.margin.to_string())
        .env("NUMBER", st.number.to_string())
        .env("CMD_PARAM", cmd_param)
        .args(["/C", bat.to_string_lossy().as_ref()])
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .spawn()
        .map(|mut c| c.wait())
        .ok();
}
