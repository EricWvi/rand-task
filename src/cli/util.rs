#![allow(dead_code)]

use rand::Rng;
use rtdb::tasks::TaskType;
use rtdb::Task;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use std::time::{SystemTime, UNIX_EPOCH};
use std::{fs, iter};
use termcolor::WriteColor;

pub fn rand_task(tasks: &Vec<Task>) -> Option<&Task> {
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
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}

pub fn open_md(file_name: &str) {
    let path = if file_name.ends_with("md") {
        let dir = rtdb::config::task_dir();
        let mut path = PathBuf::from(dir);
        let task_type = crate::TASK.get().unwrap().r#type;
        path.push(match task_type {
            TaskType::FocusAnotherThing => "focus-another-thing",
            TaskType::TakeABreak => "take-a-break",
            TaskType::Tired => "tired",
            TaskType::Today => "current-work",
            TaskType::Inbox => "inbox",
            TaskType::En => "en",
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
    use std::io::{self, Write};
    use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
    let percent = (total * 60 - min * 60 - sec) * 100 / (total * 60);
    let remainder = format!("{}m {}s", min, sec);
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
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
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

fn script_file(script: &str) -> PathBuf {
    let mut dir = std::env::temp_dir();
    let rnd_name = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    dir.push(format!("rand-task-{rnd_name}.osas"));
    fs::File::create(&dir).expect("failed to create temp file");
    fs::write(&dir, script).expect("failed to write to temp file");
    dir
}

pub fn alert(alert_type: ASCmd) -> Command {
    let dir = script_file(alert_type.script());
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
    let dir = script_file(ASCmd::LockScreen.script());
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
    use super::{open_md, turn_wifi_off};
    use rtdb::tasks::{TaskStatus, TaskType};
    use rtdb::Task;
    
    #[tokio::test]
    async fn test_open_md() {
        crate::TASK.set(Task {
            id: 0,
            name: "".to_string(),
            md_link: None,
            r#type: TaskType::Today,
            weight: 0,
            status: TaskStatus::Pending,
        });
        rtdb::init().await;
        open_md(
            "/Users/wangyi/Documents/PersonalFile/Git/Obsidian/RandTask/current-work/rand-task.md",
        );
    }
}
