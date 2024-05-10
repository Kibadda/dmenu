use freedesktop_entry_parser::parse_entry;
use std::{env, ffi::OsStr, fs, path::PathBuf};

#[derive(Clone, Debug)]
pub struct Program {
    pub name: String,
    pub cmd: Vec<String>,
}

fn is_executable(cmd: &str) -> bool {
    if let Ok(path) = env::var("PATH") {
        for p in path.split(':') {
            let p_str = format!("{}/{}", p, cmd);
            if fs::metadata(p_str).is_ok() {
                return true;
            }
        }
    }

    false
}

pub fn load_from_dir(dir: &str) -> Vec<Program> {
    let dir = PathBuf::from(dir);

    let mut programs: Vec<Program> = Vec::new();

    for file in fs::read_dir(dir).expect("could not read dir") {
        let file = file.expect("file failed");
        let path = file.path();

        if path.extension().and_then(OsStr::to_str) != Some("desktop") {
            continue;
        }

        let entry = parse_entry(path).expect("parsing failed");
        let section = entry.section("Desktop Entry");

        if let Some(exec) = section.attr("Exec") {
            if match section.attr("TryExec") {
                Some(try_exec) => is_executable(try_exec),
                None => true,
            } {
                let mut cmd: Vec<String> = Vec::new();

                if let Some(terminal) = section.attr("Terminal") {
                    if terminal == "true" {
                        cmd.push(String::from("kitty"));
                    }
                }

                exec.split_whitespace()
                    .for_each(|s| cmd.push(s.to_string()));

                programs.push(Program {
                    name: match section.attr("Name") {
                        Some(name) => name.to_owned(),
                        None => exec.to_owned(),
                    },
                    cmd,
                });
            }
        }
    }

    programs
}
