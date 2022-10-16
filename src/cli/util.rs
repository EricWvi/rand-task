use rand::Rng;
use rtdb::tasks::TaskType;
use rtdb::Task;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};

use std::time::{SystemTime, UNIX_EPOCH};
use std::{fs, iter};

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
    let dir = std::env::var("TASK_DIR").expect("TASK_DIR must be set");
    let mut path = PathBuf::from(dir);
    let task_type = crate::TASK.get().unwrap().r#type;
    path.push(match task_type {
        TaskType::FocusAnotherThing => "focus-another-thing",
        TaskType::TakeABreak => "take-a-break",
        TaskType::Tired => "tired",
        TaskType::Today => "current-work",
    });
    path.push(file_name);
    Command::new("code").arg(path).output();
}

pub fn progressing_bar(min: i32, sec: i32, total: i32) -> String {
    let percent = (total * 60 - min * 60 - sec) * 100 / (total * 60);
    let remainder = format!("{}m {}s", min, sec);
    let mut bar = "|".to_string();
    for c in iter::repeat('=').take(percent as usize / 5) {
        bar.push(c);
    }
    if percent != 100 {
        bar.push('>')
    }
    for c in iter::repeat('-').take(21 - bar.len()) {
        bar.push(c);
    }
    bar.push('|');
    format!("{bar} {percent}%, {remainder}")
}

pub enum ASCmd {
    AlertFinished,
    AlertFiveMinutes,
    DialogWithAnswer,
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
            ASCmd::DialogWithAnswer => {
                r#"on run argv
	set answer to display dialog "{0}" default answer "{1}"
	set content to text returned of answer
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
    let mut dir = script_file(alert_type.script());
    let mut cmd = Command::new("osascript");
    cmd.arg(dir.to_str().unwrap());
    cmd.stdout(Stdio::null());
    cmd
}

pub fn get_dialog_answer(title: &str, default: &str) -> String {
    let script = ASCmd::DialogWithAnswer.script().replace("{0}", title);
    let script = script.replace("{1}", default);
    let mut dir = script_file(script.as_str());
    let mut child = Command::new("osascript")
        .arg(dir.to_str().unwrap())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    child.wait();
    let mut stdout = child.stdout.take().unwrap();
    let mut answer = String::new();
    stdout.read_to_string(&mut answer);
    answer
}

pub fn lock_screen() {
    let dir = script_file(ASCmd::LockScreen.script());
    Command::new("osascript")
        .arg(dir.to_str().unwrap())
        .output();
}

pub fn turn_wifi_off() {
    Command::new("networksetup")
        .args(["-setairportpower", "en0", "off"])
        .output();
}

pub async fn send_msg(msg: &str) {
    reqwest::get(format!("http://www.pushplus.plus/send?token=d2410bf17a1547cfadf1b42687279ba2&title={msg}&content={msg}&template=html"))
        .await;
}

#[cfg(test)]
mod test {
    use super::progressing_bar;
    use crate::cli::util::{get_dialog_answer, lock_screen, turn_wifi_off};
    use std::path::PathBuf;
    use std::process::Child;
    use std::thread;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn test_progressing_bar() {
        assert_eq!(
            progressing_bar(10, 0, 10),
            "|>-------------------| 0%, 10m 0s".to_string()
        );
        assert_eq!(
            progressing_bar(5, 0, 10),
            "|==========>---------| 50%, 5m 0s".to_string()
        );
    }

    #[test]
    fn test_script() {
        get_dialog_answer("xx", "");
        // println!("{}!", );
    }
}
