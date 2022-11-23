#![allow(dead_code)]

use rand::Rng;
use rtdb::projects::ProjectType;
use rtdb::{task_dao, Project};
use std::path::PathBuf;
use std::process::{Command, Stdio};

use std::iter;
use std::sync::Mutex;

pub fn rand_task(tasks: &Vec<Project>) -> Option<&Project> {
    let mut total = 0;
    for t in tasks {
        total += t.weight;
    }
    let mut rng = rand::thread_rng();
    let mut rnd = rng.gen_range(0..total);
    for (i, t) in tasks.iter().enumerate() {
        if rnd < t.weight {
            return Some(&tasks[i]);
        }
        rnd -= t.weight;
    }
    None
}

#[inline]
pub fn get_input() -> String {
    rtdb::util::get_input()
}

pub async fn set_global_task(project: &Project) {
    let task = match task_dao::get_first_task(rtdb::db(), project.id).await {
        Ok(t) => t,
        Err(e) => panic!("{e:?}"),
    };
    if task.is_some() {
        println!("Task: {} - {}", project.name, task.as_ref().unwrap().name);
        if crate::TASK.get().is_none() {
            crate::TASK
                .set(Mutex::new(task.unwrap()))
                .expect("failed to set global TASK");
        } else {
            let mut t = crate::TASK.get().unwrap().lock().unwrap();
            *t = task.unwrap();
        }
    } else {
        println!("Task: {}", project.name);
        print!("\u{001b}[37;41;1m WARN \u{001b}[0m");
        println!(" the project has no subtasks");
        if crate::TASK.get().is_some() {
            let mut t = crate::TASK.get().unwrap().lock().unwrap();
            t.id = 0;
        }
    }
}

pub fn set_global_project(project: &Project) {
    crate::PROJECT
        .set(project.clone())
        .expect("failed to set global PROJECT");
}

pub fn open_link(file_name: &str) {
    let path = if file_name.ends_with("md") {
        let dir = rtdb::config::project_dir();
        let mut path = PathBuf::from(dir);
        let project_type = crate::PROJECT.get().unwrap().r#type;
        path.push(match project_type {
            ProjectType::FocusAnotherThing => "focus-another-thing",
            ProjectType::TakeABreak => "take-a-break",
            ProjectType::Tired => "tired",
            ProjectType::Today => "current-work",
            ProjectType::Inbox => "inbox",
            ProjectType::En => "en",
        });
        path.push(file_name);
        path
    } else {
        PathBuf::from(file_name)
    };
    Command::new("open")
        .arg(path)
        .output()
        .expect("failed to execute open_md");
}

pub fn progressing_bar(min: i32, sec: i32, total: i32) -> std::io::Result<()> {
    use std::io::Write;
    use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
    let percent = (total * 60 - min * 60 - sec) * 100 / (total * 60);
    let remainder = format!("{}m {}s  ", min, sec);
    let mut color_bar = String::new();
    for c in iter::repeat('━').take((percent as f64 / 2.5) as usize) {
        color_bar.push(c);
    }
    if percent != 100 {
        color_bar.push('╸')
    }
    let mut bar_end = String::new();
    for c in iter::repeat('━').take(41 - color_bar.chars().count()) {
        bar_end.push(c);
    }
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(if percent != 100 {
        Color::Red
    } else {
        Color::Green
    })))?;
    write!(&mut stdout, "\r{color_bar}")?;
    stdout.reset()?;
    print!("{bar_end} {percent}%, ");
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))?;
    print!("{remainder}");
    stdout.reset()?;
    Ok(())
}

pub enum ASCmd {
    AlertFinished,
    AlertFiveMinutes,
    LockScreen,
}

impl ASCmd {
    fn script(&self) -> &'static str {
        match self {
            ASCmd::AlertFinished => {
                r#"on run argv
	set theDialogText to "Time up!"
	display alert theDialogText
end run"#
            }
            ASCmd::AlertFiveMinutes => {
                r#"on run argv
	set theDialogText to "There are 5 minutes remaining!"
	display alert theDialogText
end run"#
            }
            ASCmd::LockScreen => {
                r#"on run argv
	activate application "SystemUIServer"
	tell application "System Events"
		tell process "SystemUIServer" to keystroke "q" using {command down, control down}
	end tell
	tell application "Finder" to sleep
end run"#
            }
        }
    }
}

pub fn alert(alert_type: ASCmd) -> Command {
    let dir = rtdb::util::script_file(alert_type.script());
    let mut cmd = Command::new("osascript");
    cmd.arg(dir.to_str().unwrap());
    cmd.stdout(Stdio::null());
    cmd
}

pub fn get_dialog_answer(title: &str, default: &str) -> String {
    rtdb::util::get_dialog_answer(title, default)
}

pub fn eval_choice(choices: i32, new_line: bool) -> char {
    rtdb::util::eval_choice(choices, new_line)
}

pub fn lock_screen() {
    let dir = rtdb::util::script_file(ASCmd::LockScreen.script());
    Command::new("osascript")
        .arg(dir.to_str().unwrap())
        .output()
        .expect("failed to execute lock_screen");
}

pub fn turn_wifi_off() {
    Command::new("networksetup")
        .args(["-setairportpower", "en0", "off"])
        .output()
        .expect("failed to execute turn_wifi_off");
}

pub async fn send_msg(msg: &str) {
    reqwest::get(format!("http://www.pushplus.plus/send?token=d2410bf17a1547cfadf1b42687279ba2&title={msg}&content={msg}&template=html"))
        .await.expect("failed to send request to pushplus");
}

#[cfg(test)]
mod test {
    use super::progressing_bar;
    use super::{open_link, turn_wifi_off};
    use rtdb::projects::{ProjectStatus, ProjectType};
    use rtdb::Project;

    #[tokio::test]
    async fn test_open_md() {
        crate::PROJECT.set(Project {
            id: 0,
            name: "".to_string(),
            md_link: None,
            r#type: ProjectType::Today,
            weight: 0,
            status: ProjectStatus::Pending,
        });
        rtdb::init().await;
        open_link(
            "/Users/wangyi/Documents/PersonalFile/Git/Obsidian/RandTask/current-work/rand-task.md",
        );
    }

    #[tokio::test]
    async fn test_print_color() {
        println!("\u{001b}[31;1mHelloWorld\u{001b}[0m");
        println!("\u{001b}[37;41;1m WARN \u{001b}[0m");
    }
}
